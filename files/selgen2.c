#define N 12
#define I 1

/* How much memory are we allowed to allocate from the heap? */
/* #define MAX_N_BYTES  (1024 * 1024) */
#define MAX_N_BYTES (1000 * 1024 * 1024)

/* How much work will we spend on graph (orderset_t) isomorphism
   recognition code? */
#define N_TOPO_ITERATIONS 3

#define N_TOPO_ITERATIONS2 2

/* A value larger or equal to `ceil(log_2(N))' */
#define BITS_IN_N 4

typedef unsigned int cost_t;
#define COST_INFINITE 0x3FF

/* Current cost at the root of the search tree. */
cost_t current_search_cost;

/* Parameters that control the format of output. */
#define VAR_NAME(i) ("abcdefghijklmnopqrstuvxyz"[i])

typedef unsigned int uint32_t;

#include <assert.h>
#include <malloc.h>
#include <math.h>
#include <readline/readline.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/times.h>
#include <unistd.h>

typedef enum { UNTESTED = 0, GREATER, LESS, N_ORDERS } order_t;

static order_t opposite[N_ORDERS] = {UNTESTED, LESS, GREATER};
static const char *order_name[N_ORDERS] = {"UNTESTED", "GREATER", "LESS"};
static const char order_char[N_ORDERS] = {' ', '>', '<'};

struct parents_t;

typedef struct orderset_t {
  /* Non-zero iff this orderset has been solved. */
  unsigned int solved : 1;
  /* Non-zero iff this orderset can not be removed from the cache. */
  unsigned int locked : 1;
  /* The two variables to optimally compare in this orderset. */
  unsigned int a : BITS_IN_N;
  unsigned int b : BITS_IN_N;
  /* How many variables are there in this orderset.  (The algorithm
     discards uninteresting ones.) */
  unsigned char n : BITS_IN_N;
  /* Which variable was asked from this orderset? 0 < i < n */
  unsigned char i : BITS_IN_N;
  /* Cost for solving this orderset, or if this orderset has not been
     solved, then a cost that is not sufficient to solve this
     orderset. */
  cost_t cost : 10;

#define N_PROBES 4
#define MAX_PRIORITY 0xFFFF
#define INITIAL_PRIORITY 0xFF
  unsigned int priority : 16;

  /* This is used to get some theoretically human-readable output. */
  unsigned int number : 16;

#define N_CMPS ((((N * (N - 1)) / 2) + 15) / 16)
  uint32_t order[N_CMPS];
} orderset_t;

/* Forward declaration. */
static cost_t search(orderset_t *, cost_t);

static int order_offset[N];

static void init_offsets(void) {
  int s = -1, i;

  for (i = 0; i < N; i++) {
    order_offset[i] = s;
    s += N - i - 2;
  }
}

static inline void set_order(orderset_t *orderset, int a, int b, order_t order) {
  int n = orderset->n;
  unsigned int ix, shmt;
  uint32_t mask;

  assert(a != b);
  assert(a < n);
  assert(b < n);
  /* assert(order > UNTESTED); */

  if (a < b)
    ix = order_offset[a] + b;
  else {
    ix = order_offset[b] + a;
    order = opposite[order];
  }
  shmt = (ix % 16) * 2;
  ix = ix / 16;
  mask = 0x3 << shmt;

  assert(ix < N_CMPS);
  assert(shmt < 32);

  orderset->order[ix] = (orderset->order[ix] & ~mask) | (order << shmt);
}

static inline order_t get_order(orderset_t *orderset, int a, int b) {
  int n = orderset->n;
  order_t order;
  unsigned int ix, shmt;
  uint32_t mask;

  assert(a < n);
  assert(b < n);

  if (a == b) {
    assert(0);
    return N_ORDERS;
  }

  if (a < b)
    ix = order_offset[a] + b;
  else
    ix = order_offset[b] + a;
  shmt = (ix % 16) * 2;
  ix = ix / 16;
  mask = 0x3;
  order = (orderset->order[ix] >> shmt) & mask;
  if (a > b) order = opposite[order];

  assert(order < N_ORDERS);

  return order;
}

static inline void copy_orderset(orderset_t *to, orderset_t *from) {
  assert(to != NULL);
  assert(from != NULL);

  memcpy(to, from, sizeof(orderset_t));
  to->locked = 0;
}

static inline int orderset_is_equal(orderset_t *x, orderset_t *y) {
  if (x == y) return 1;
  if (x->n != y->n || x->i != y->i || memcmp(x->order, y->order, N_CMPS * sizeof(x->order[0]))) return 0;
  return 1;
}

static void inline clear_orderset(orderset_t *orderset) { memset(orderset, 0, sizeof(*orderset)); }

/* Return non-zero on success, zero on failure. */
static int set_order_and_closure(orderset_t *orderset, int a, int b, order_t order) {
  int i, n = orderset->n;
  order_t old_order = get_order(orderset, a, b);

  switch (order) {
    case LESS:
      switch (old_order) {
        case LESS:
          break;
        case UNTESTED:
          set_order(orderset, a, b, LESS);
          for (i = 0; i < n; i++)
            if (i != a && i != b) {
              switch (get_order(orderset, b, i)) {
                case LESS:
                  if (!set_order_and_closure(orderset, a, i, LESS)) return 0;
                  break;
              }
              switch (get_order(orderset, a, i)) {
                case GREATER:
                  if (!set_order_and_closure(orderset, b, i, GREATER)) return 0;
                  break;
              }
            }
          break;
        default:
          return 0;
          break;
      }
      break;
    case GREATER:
      switch (old_order) {
        case GREATER:
          break;
        case UNTESTED:
          set_order(orderset, a, b, GREATER);
          for (i = 0; i < n; i++)
            if (i != a && i != b) {
              switch (get_order(orderset, b, i)) {
                case GREATER:
                  if (!set_order_and_closure(orderset, a, i, GREATER)) return 0;
                  break;
              }
              switch (get_order(orderset, a, i)) {
                case LESS:
                  if (!set_order_and_closure(orderset, b, i, LESS)) return 0;
                  break;
              }
            }
          break;
        default:
          return 0;
          break;
      }
      break;
  }

  return 1;
}

static int orderset_selects_ith(orderset_t *orderset, int i, unsigned char *varp) {
  int a, b, n = orderset->n, n_orders[N_ORDERS], j, slop;

  for (a = 0; a < N; a++) {
    for (j = 0; j < N_ORDERS; j++) n_orders[j] = 0;

    for (b = 0; b < N; b++)
      if (a != b) n_orders[get_order(orderset, b, a)]++;

    slop = 0;
    if (n_orders[GREATER] + n_orders[UNTESTED] <= i && i + n_orders[UNTESTED] <= n_orders[GREATER] + slop) {
      if (varp != NULL) *varp = a;
      return 1;
    }
  }
  return 0;
}

/* min_n_comparisons[n][i] contains the lowest number of comparisons
   required to solve the corresponding selection problem from the
   beginning.  Contains zero if no lower bound known. */
static unsigned char min_n_comparisons[1 << BITS_IN_N][1 << BITS_IN_N] = {
    /* i=1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16 */
    /* n= 0 */ {},
    /* n= 1 */ {},
    /* n= 2 */ {1, 1},
    /* n= 3 */ {2, 3, 2},
    /* n= 4 */ {3, 4, 4, 3},
    /* n= 5 */ {4, 6, 6, 6, 4},
    /* n= 6 */ {5, 7, 8, 8, 7, 5},
    /* n= 7 */ {6, 8, 10, 10, 10, 8, 6},
    /* n= 8 */ {7, 9, 11, 12, 12, 11, 9, 7},
    /* n= 9 */ {8, 11, 12, 14, 14, 14, 12, 11, 8},
    /* n=10 */ {9, 12, 14, 15, 16, 16, 15, 14, 12, 9},
    /* n=11 */ {10, 13, 15, 17, 18, 18, 18, 17, 15, 13, 10},
    /* n=12 */ {11, 14, 17, 18, 19, 21, 21, 19, 18, 17, 14, 11},
};

orderset_t *hash_table = NULL;
uint32_t hash_table_size;

unsigned long long n_hits = 0, n_misses = 0;

static inline unsigned int cache_value(unsigned int n, unsigned int i) {
#if 1
  int shmt = min_n_comparisons[n][i];

  if (shmt > current_search_cost) shmt = current_search_cost;

  return shmt; /* 48.66s */
#endif
}

static inline uint32_t orderset_hash(orderset_t *orderset) {
  uint32_t h = (orderset->n << 8) | orderset->i;
  int i;

  for (i = 0; i < N_CMPS; i++) {
    h += orderset->order[i] >> 20;
    h += h << 10;
    h ^= h >> 7;
    h += orderset->order[i] >> 9;
    h += h << 10;
    h ^= h >> 7;
    h += orderset->order[i];
    h += h << 10;
    h ^= h >> 7;
  }
  h += h << 3;
  h ^= h >> 11;
  h += h << 15;
  return h;
}

static orderset_t *probe_orderset_cache(orderset_t *orderset, orderset_t **alts) {
  orderset_t *alts_reserve[N_PROBES];
  orderset_t *p;
  int i, j;
  uint32_t h;
  /* Some random data to prevent possible infinite loops, taken from
     `cat /dev/random | od -x' */
#define N_RNDS 16
  static uint32_t rnd[N_RNDS] = {0x33432BD5, 0xD9DA2857, 0xC16CA349, 0xB48E786F, 0xC280E04C, 0xB6419EF7,
                                 0x35E68035, 0x7327028D, 0x1618C293, 0x9E0A336E, 0x1709F35C, 0x5102EB06,
                                 0x2B02AD5B, 0x8E4D16D4, 0x2DEA4A71, 0x2A3DDDC0};
  int rnd_ix = 0;

  if (hash_table == NULL) {
    hash_table_size = MAX_N_BYTES / sizeof(orderset_t);
    hash_table = calloc(hash_table_size, sizeof(orderset_t));
    if (hash_table == NULL) {
      fprintf(stderr,
              "Malloc failed when allocating the %u-entry hash table.  "
              "Sorry.\n",
              hash_table_size);
      exit(1);
    }
  }

  h = orderset_hash(orderset);

  if (alts == NULL) alts = alts_reserve;
  for (i = 0; i < N_PROBES; i++) alts[i] = NULL;
  for (i = 0; i < N_PROBES; i++) {
    p = &hash_table[h % hash_table_size];
    if (orderset_is_equal(p, orderset)) {
      /* Hit!  Increment priority and return. */
      unsigned long v = cache_value(p->n, p->i);
      if (p->priority <= MAX_PRIORITY - v) p->priority += v;
      n_hits++;
      return p;
    } else if (!p->locked) {
      for (j = 0; j < i; j++)
        if (alts[j] == p) {
          /* We don't want to identical pointers in `alts'.  Stir up
             `h' and pick another slot in the hash table. */
          h ^= rnd[rnd_ix++];
          if (rnd_ix == N_RNDS) rnd_ix = 0;
          i--;
          goto next_probe;
        }
      alts[i] = p;
    }
    h += h << 3;
    h ^= h >> 11;
    h += h << 15;
  next_probe:;
  }

  /* Miss. */
  n_misses++;
  return NULL;
}

static inline void store_orderset_cache(orderset_t *orderset, orderset_t **alts) {
  int i, j;

  for (i = j = 0; i < N_PROBES; i++) {
    if (alts[i] == NULL) {
      if (i == j) j++;
      continue;
    }
    if (i == j || alts[j]->priority > alts[i]->priority) j = i;
  }
  if (j < N_PROBES && alts[j] != NULL) {
    copy_orderset(alts[j], orderset);
    alts[j]->priority = cache_value(orderset->n, orderset->i);
  }

  /* Decrease weight of all other alternatives.
   */
  for (i = 0; i < N_PROBES; i++)
    if (i != j && alts[i] != NULL && alts[i]->priority > 1) alts[i]->priority--;
}

/* Perform various simplifications and canonifications in the
   orderset, and detects whether the orderset has become solved. */
static inline void canonify_orderset(orderset_t *orderset, unsigned char *new_a) {
  unsigned char dropped[N], new_a_reserve[N], n_orders[N][N_ORDERS];
  unsigned int lessness[N], n_topo_iterations;
  order_t order;
  uint32_t topo_hash_old[N], topo_hash_new[N], h;
  int n = orderset->n, new_n, n_less_dropped, a, b, c, j;
  int have_dropped, n_cmps;
  orderset_t new_orderset;

  if (new_a == NULL) new_a = new_a_reserve;

  /* Compute the n_orders array.  Given some order, say `GREATER', and
     some variable `a', then `n_orders[a][GREATER]' is computed below
     to tell how many other variables are known to be greater than `a'. */
  memset(n_orders, 0, sizeof(n_orders));
  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++) {
      order_t order = get_order(orderset, b, a);
      n_orders[a][order]++;
      n_orders[b][opposite[order]]++;
    }

  /* Mark into `dropped' each variable which is either greater that
     `i' other variables or less than `n-i' other variables. */
  memset(dropped, 0, sizeof(dropped));
  for (n_less_dropped = a = 0; a < n; a++)
    if (n_orders[a][GREATER] > orderset->i)
      dropped[a] = 1;
    else if (n_orders[a][LESS] >= n - orderset->i) {
      dropped[a] = 1;
      n_less_dropped++;
    }

  /* Compute into `new_n' the number of variables not dropped and let
     `new_a[a]' be the new name (number) of variable `a'.  The `new_a'
     is an indirection that will be worked on further. */
  for (new_n = a = 0, b = n - 1; a < n; a++)
    if (!dropped[a])
      new_a[new_n++] = a;
    else
      new_a[b--] = a;

  if (new_n == n)
    have_dropped = 0;
  else {
    have_dropped = 1;
    n = new_n; /* `new_n' is a cumbersome name... */
    /* We have dropped some variables, therefore recompute the
       `n_orders'-array.  Since it is used later in the orderset
       isomorphism/canonifying code, we might otherwise fail to detect
       the isomorphism of two ordersets, one of which was constructed
       by dropping a variable. */
    memset(n_orders, 0, sizeof(n_orders));
    for (a = 0; a < n - 1; a++)
      for (b = a + 1; b < n; b++) {
        order_t order = get_order(orderset, new_a[b], new_a[a]);
        n_orders[new_a[a]][order]++;
        n_orders[new_a[b]][opposite[order]]++;
      }
  }

  /* Sort the variables into an as canonical order as possible
     according to a number of characteristics:
      - The variable's "lessness" based on already performed
        comparisons (and their transitive closure).
      - Hash values based on the lessness of the variable (`new_a[a]')
        and the relative order and lessness of the other variables
        (`new_a[b]') to which the first variable (`new_a[a]') has an
        established order.
      - Hash values based on the previously computed hash value of the
        variable (`new_a[a]') and the relative order and previously
        computed hash values of the other variables (`new_a[b]') to
        which the first variable (`new_a[a]') has an established
        order.
      - The hash values are recomputed a total of `N_TOPO_ITERATIONS'
        times, where each iteration propagates the topology of the
        graph to the variables.
     After computing the lessnesses and hash values of variables, we
     sort the variables according to increasing lessness and hash
     value.  But this still leaves one significant kind of ordering
     unsolved.  Consider the case of `a>b,c>d'.  The sorting can
     produce either the order `abcd' or `abdc' (`bacd' and `badc' will
     become either one in the later variable renaming).  Therefore, in
     the last pass we do the following:
       - if two or more variables have the same lessness and hash
         values, and if so, we pin their relative order, and use their
         relative orders to rehash variables which have not yet been
         pinned.

     It is still possible that some graph (orderset_t) isomorphishms
     go undetected:
       - hash values are only 32-bit and they can always collide,
         particularly since we can not use arbitrary hash functions,
         but have to choose commutative ones.
       - the last pass performs only one recomputation of the hash
         values, therefore it is possible that isomorphic "zig-zag" or
         "fence-like" orders (`a>b,a>d,c>b,c>d'+same again) do
         not become fully canonified.
     But I hope they would be so infrequent that they cause little
     excess computation. */

  for (a = 0; a < n; a++)
    topo_hash_old[new_a[a]] = lessness[new_a[a]] =
        N * N - (n_orders[new_a[a]][GREATER] << BITS_IN_N) - n_orders[new_a[a]][LESS];

  for (n_topo_iterations = 0; n_topo_iterations < N_TOPO_ITERATIONS; n_topo_iterations++) {
    /* Compute new hash values based on the old ones. */
    for (a = 0; a < n; a++) {
      h = topo_hash_old[new_a[a]];
      h += h << 10;
      for (b = 0; b < n; b++)
        if (a != b && (order = get_order(orderset, new_a[a], new_a[b])) != UNTESTED)
          h += (topo_hash_old[new_a[b]] << 4) + order;
      /* NOTE: This hash function must be commutative w.r.t. all
         other compared variables `new_a[b]'.  Therefore we can
         *not* issue the typical `h += h << 10; h ^= h >> 7;' here. */
      topo_hash_new[new_a[a]] = h;
    }

    /* Propagate new hash values to become the old ones in the next
       iteration. */
    for (a = 0; a < n; a++) topo_hash_old[new_a[a]] = topo_hash_new[new_a[a]];
  }

  /* A simple O(n^2) sort based on the lessness and hash values. */
  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++)
      if (lessness[new_a[a]] < lessness[new_a[b]] ||
          (lessness[new_a[a]] == lessness[new_a[b]] && topo_hash_old[new_a[a]] < topo_hash_old[new_a[b]])) {
        j = new_a[a];
        new_a[a] = new_a[b];
        new_a[b] = j;
      }

  /* The final pass. */
  for (a = 1; a < n; a++)
    if (lessness[new_a[a]] == lessness[new_a[a - 1]] && topo_hash_old[new_a[a]] == topo_hash_old[new_a[a - 1]]) {
      /* Skip over the equivalent variables. */
      for (a++; a < n && lessness[new_a[a]] == lessness[new_a[a - 1]] &&
                topo_hash_old[new_a[a]] == topo_hash_old[new_a[a - 1]];
           a++)
        ;
      if (a == n) continue;
      for (n_topo_iterations = 0; n_topo_iterations < N_TOPO_ITERATIONS2; n_topo_iterations++) {
        /* Recompute the hash values. */
        for (c = a; c < n; c++) {
          h = topo_hash_old[new_a[c]];
          h += h << 10;
          for (b = 0; b < a; b++)
            if ((order = get_order(orderset, new_a[c], new_a[b])) != UNTESTED)
              h += (topo_hash_old[new_a[b]] << (4 + BITS_IN_N)) - (order << BITS_IN_N) + b;
          topo_hash_new[new_a[c]] = h;
        }
        /* Propagate hash values in case we should hash again. */
        for (c = a; c < n; c++) topo_hash_old[new_a[c]] = topo_hash_new[new_a[c]];
      }
      /* Re-sort the variables below the equivalent ones we just
         skipped over. */
      for (c = a; c < n - 1; c++)
        for (b = c + 1; b < n; b++)
          if (lessness[new_a[c]] < lessness[new_a[b]] ||
              (lessness[new_a[c]] == lessness[new_a[b]] && topo_hash_old[new_a[c]] < topo_hash_old[new_a[b]])) {
            j = new_a[c];
            new_a[c] = new_a[b];
            new_a[b] = j;
          }
    }

  /* Construct a new orderset based on `new_a' and `n'. */
  clear_orderset(&new_orderset);
  new_orderset.n = n;
  new_orderset.i = orderset->i - n_less_dropped;
  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++) set_order(&new_orderset, a, b, get_order(orderset, new_a[a], new_a[b]));
  copy_orderset(orderset, &new_orderset);

  /* Detect if the problem became solved, and feed the killer
     heuristic with a guesstimate of how difficult a problem we have
     left. */
  if (n == 1) orderset->solved = 1;
}

static unsigned int inline highest_bit(uint32_t x) {
  unsigned int i;

  if (x-- == 0) return 0;
  for (i = 0; x != 0; i++) x >>= 1;
  return i;
}

typedef int n_orders_t[N][N_ORDERS];

typedef enum { SOLVED, IMPOSSIBLE, UNCLEAR } solvability_t;

/* Return non-zero if the given orderset might be solvable in
   `cost_limit' comparisons.  Returns zero only if it is completely
   certain that a solution can not be found in the given number of
   comparisons. */

static inline int estimate_hardness(orderset_t *orderset, int *hardness, int cost_limit, int changed) {
  int n_orders[N][N_ORDERS];
  int n = orderset->n, a, b, c;
  orderset_t os;
  cost_t cost;

  /* Compute the n_orders array.  Given some order, say `GREATER', and
     some variable `a', then `n_orders[a][GREATER]' is computed below
     to tell how many other variables are known to be greater than `a'. */
  memset(n_orders, 0, sizeof(n_orders));
  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++) {
      order_t order = get_order(orderset, b, a);
      n_orders[a][order]++;
      n_orders[b][opposite[order]]++;
    }

  if (hardness != NULL) {
    *hardness = 0;
    for (a = 0; a < n; a++) {
      int d = n_orders[a][GREATER] - n_orders[a][LESS];
      int u = n_orders[a][UNTESTED];
      if (d < 0) d = -d;
      *hardness -= d + 2 * u;
    }
  }

  if (orderset->i == 0 || orderset->i == n - 1) return cost_limit >= n - 1;
  /* The following two estimates are based on a lower bound given in
     TAOCP, Volume 3, Section 5.3.3, Exercise 6. */
  else if (orderset->i == 1) {
    uint32_t n_groups = 0, s = 0;
    for (a = 0; a < n; a++)
      if (n_orders[a][GREATER] == 0) {
        n_groups++;
        s += 1 << n_orders[a][LESS];
      }
    return cost_limit >= n_groups - 2 + highest_bit(s);
  } else if (orderset->i == n - 2) {
    uint32_t n_groups = 0, s = 0;
    for (a = 0; a < n; a++)
      if (n_orders[a][LESS] == 0) {
        n_groups++;
        s += 1 << n_orders[a][GREATER];
      }
    return cost_limit >= n_groups - 2 + highest_bit(s);
  } else {
    int n_cmps = min_n_comparisons[n][orderset->i];

    /* Subtract from `n_cmps' every order which is not deducible from
       some other order.  Try first the most easily computed ones. */
    for (a = 0; a < n; a++)
      if (n_orders[a][LESS] == 1) {
        n_cmps--;
        if (n_cmps <= cost_limit) goto out;
      }
    for (a = 0; a < n - 1; a++)
      if (n_orders[a][LESS] >= 2)
        for (b = a + 1; b < n; b++) {
          if (get_order(orderset, b, a) == LESS)
            if (n_orders[b][GREATER] == 1) {
              n_cmps--;
              if (n_cmps <= cost_limit) goto out;
            } else {
              for (c = 0; c < n; c++)
                if (a != c && b != c && get_order(orderset, b, c) == LESS && get_order(orderset, c, a) == LESS)
                  goto next_b;
              n_cmps--;
              if (n_cmps <= cost_limit) goto out;
            next_b:;
            }
        }
  out:
    if (n_cmps > cost_limit) return 0;
  }

  return 1;
}

typedef struct {
  unsigned long hash : 28;
  unsigned int n_hits : 4;
} fw_cache_entry_t;

#define N_FW_CACHE_ENTRIES (1024 * 1024)

static fw_cache_entry_t fw_cache[N_FW_CACHE_ENTRIES];

static inline solvability_t fw_search(orderset_t *orderset, n_orders_t n_orders, int initial_a, int initial_b,
                                      int cost_limit, int depth) {
  int a, b, d, n = orderset->n;
  orderset_t os, *cached_os;
  solvability_t s;

  copy_orderset(&os, orderset);
  if (initial_a != 0 || initial_b != 0) {
    canonify_orderset(&os, NULL);
    if (!estimate_hardness(&os, NULL, cost_limit, 0)) return IMPOSSIBLE;
    cached_os = probe_orderset_cache(&os, NULL);
    if (cached_os != NULL)
      if (cached_os->solved)
        if (cached_os->cost <= cost_limit)
          return SOLVED;
        else
          return IMPOSSIBLE;
      else if (cached_os->cost >= cost_limit)
        return IMPOSSIBLE;
      else
        goto unclear;
  }

  /* Experiments showed there's little benefit from depth-limiting the
     loop below. */
  for (a = initial_a; a < n; a++)
    if (n_orders[a][LESS] == 0 && n_orders[a][GREATER] >= 2)
      for (b = (a == initial_a ? initial_b : 0); b < n; b++)
        if (a != b && n_orders[b][GREATER] == 0 &&
            n_orders[b][LESS] >= 2
            /* && (d = n_orders[a][GREATER] + n_orders[b][LESS]) >= 5 */
            && get_order(orderset, a, b) == UNTESTED) {
          set_order(orderset, a, b, LESS);
          s = fw_search(orderset, n_orders, a, b + 1, cost_limit, depth + 1);
          set_order(orderset, a, b, UNTESTED);
          if (s == IMPOSSIBLE) return IMPOSSIBLE;
        }

unclear:
  if (initial_a != 0 || initial_b != 0) {
    uint32_t h = orderset_hash(&os);
    uint32_t i = h % N_FW_CACHE_ENTRIES;

    if (fw_cache[i].hash == (h >> 4)) {
      if (fw_cache[i].n_hits >= 6 && depth <= 4) {
        int cost = search(&os, cost_limit);
        if (cost > cost_limit) return IMPOSSIBLE;
      } else if (fw_cache[i].n_hits < 15)
        fw_cache[i].n_hits++;
    } else {
      fw_cache[i].hash = h >> 4;
      fw_cache[i].n_hits = 1;
    }
  }

  return UNCLEAR;
}

static unsigned long long n_nodes = 0;
static unsigned long nth_orderset = 0;

static cost_t search(orderset_t *parent, cost_t cost_limit) {
  orderset_t *alts[N_PROBES], *cached_parent, child[N_ORDERS];
  cost_t cost, child_cost;
  order_t order, killer_order;
  int a, b, c, d, hardness, worst_hardness, n, new_n, delta = 0;

  if (parent->solved) return parent->cost;
  if (cost_limit <= 0) return COST_INFINITE;
  cached_parent = probe_orderset_cache(parent, alts);
  if (cached_parent != NULL) {
    if (cached_parent->solved) return cached_parent->cost;
    if (cached_parent->cost >= cost_limit) return COST_INFINITE;
    cached_parent->locked = 1;
  }

  n = parent->n;

  {
    n_orders_t n_orders;

    memset(n_orders, 0, sizeof(n_orders));
    for (a = 0; a < n - 1; a++)
      for (b = a + 1; b < n; b++) {
        order_t order = get_order(parent, b, a);
        n_orders[a][order]++;
        n_orders[b][opposite[order]]++;
      }
    if (fw_search(parent, n_orders, 0, 0, cost_limit, 0) == IMPOSSIBLE) return COST_INFINITE;
  }

  if (cached_parent != NULL) parent = cached_parent;

  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++)
      if (get_order(parent, a, b) == UNTESTED) {
        worst_hardness = -1000000;
        killer_order = GREATER;
        for (order = GREATER; order <= LESS; order++) {
          copy_orderset(&child[order], parent);
          set_order_and_closure(&child[order], a, b, order);
          canonify_orderset(&child[order], NULL);
          if (!estimate_hardness(&child[order], &hardness, cost_limit - 1 - delta, n != child[order].n))
            goto next_comparison;
          {
            orderset_t *cached_child = probe_orderset_cache(&child[order], NULL);
            if (cached_child != NULL) {
              if (!cached_child->solved && cached_child->cost >= cost_limit - 1 - delta) goto next_comparison;
            }
          }
          if (hardness > worst_hardness) {
            worst_hardness = hardness;
            killer_order = order;
          }
        }

        cost = 0;
        for (order = UNTESTED; order <= LESS; order++) {
          if (order == UNTESTED)
            child_cost = search(&child[killer_order], cost_limit - 1 - delta);
          else if (order == killer_order)
            continue;
          else
            child_cost = search(&child[order], cost_limit - 1 - delta);
          if (child_cost > cost_limit - 1 - delta) /* This test is equivalent to an alpha-beta cutoff. */
            goto next_comparison;
          child_cost++;
          if (child_cost > cost) cost = child_cost;
        }
        if (cost <= cost_limit) {
          /* The `delta' is a little trick to cause the comparison to
             most preferably occur on the first variables, thereby
             slightly improving readability of the resulting
             algorithms.  This also corresponds to alpha-beta
             windowing. */
          parent->solved = 1;
          parent->cost = cost;
          parent->a = a;
          parent->b = b;
          delta++;
        }
      next_comparison:;
      }

  n_nodes++;

  if (!parent->solved) parent->cost = cost_limit;
  if (cached_parent == NULL)
    store_orderset_cache(parent, alts);
  else
    parent->locked = 0;
  if (parent->solved)
    return parent->cost;
  else
    return COST_INFINITE;
}

static unsigned int number = 1;

/* Returns wall clock time in seconds since the beginning of the
   program.  Typical use involves two calls; one before and one after
   the benchmark, and the consumed time is the difference of these two
   times. */

static double give_time(void) {
  struct tms tms;
  long ticks_per_second;
  long ticks;

  ticks = times(&tms);
  if (ticks == -1) {
    fprintf(stderr, "'times' failed\n");
    exit(1);
  }
  ticks_per_second = sysconf(_SC_CLK_TCK);
  if (ticks_per_second == -1) {
    fprintf(stderr, "'sysconf(_SC_CLK_TCK)' failed.\n");
    exit(1);
  }
  return ticks / (double)ticks_per_second;
}

int main(int argc, char **argv) {
  orderset_t initial;
  cost_t found_cost;
  double t_start, t_end;
  unsigned char new_a[N];
  int a;

  init_offsets();

  t_start = give_time();
  clear_orderset(&initial);
  initial.n = N;
  initial.i = I - 1;

  canonify_orderset(&initial, NULL);
  if (initial.solved)
    found_cost = 0;
  else {
    current_search_cost = 1;
    do {
      /* n_nodes = n_hits = n_misses = 0; */
      found_cost = search(&initial, current_search_cost);
    } while (found_cost > current_search_cost++);
  }

  t_end = give_time();
  fprintf(stderr, "solved V(%d,%d) = %d vs %d\n", N, I, found_cost, min_n_comparisons[N][I - 1]);

  printf("# cost = %d\n", (int)found_cost);
  printf("# n_nodes = %llu\n", n_nodes);
  printf("# time = %.3f\n", t_end - t_start);
  {
    unsigned long long priority_sum = 0, cost_sum = 0;
    unsigned int i, n_solved = 0, n_unsolved = 0;
    orderset_t *p;

    for (i = 0; i < hash_table_size; i++) {
      p = &hash_table[i];
      if (p->cost == 0) /* Empty. */
        continue;
      if (p->solved)
        n_solved++;
      else
        n_unsolved++;
      priority_sum += p->priority;
      cost_sum += p->cost;
    }
    printf("# hash_table_size = %u\n", hash_table_size);
    printf("# n_solved = %u\n", n_solved);
    printf("# n_unsolved = %u\n", n_unsolved);
    printf("# n_hits = %llu\n", n_hits);
    printf("# n_misses = %llu\n", n_misses);
    printf("# avg_priority = %.3f\n", priority_sum / (double)(n_solved + n_unsolved));
    printf("# avg_cost = %.3f\n", cost_sum / (double)(n_solved + n_unsolved));
  }

  fflush(stdout);

  return 0;
}
