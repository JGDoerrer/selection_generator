/* A program that generates optimal selection functions.
 *
 * Copyright (c) 2002 Kenneth Oksanen <cessu@iki.fi>
 * 
 * This file is distributed under the GNU General Public License
 * version 2 or newer.
 */

/* Begin of configuration section. */

/* Maximum number of elements to select from. */
#ifndef N
#define N  12
#endif

#ifndef I
#define I  5
#endif

/* How much memory are we allowed to allocate from the heap? */
/* #define MAX_N_BYTES  (1024 * 1024) */
#define MAX_N_BYTES  (400 * 1024 * 1024)

/* How much work will we spend on graph (orderset_t) isomorphism
   recognition code? */
#define N_TOPO_ITERATIONS  3

#define N_TOPO_ITERATIONS2  2

/* Shall we automatically perform the pairwise comparisons?
 */
/* #define ASSUME_INITIAL_PAIRWISE */

/* Is it possible that to items are equal?  Define this macro if yes.
 */
/* #define MAY_BE_EQUAL */

/* Is it possible that to items may have undefined order, i.e. the
   input set is only partially ordered.  Define this macro if yes. */
/* #define MAY_BE_UNDEFINED */

/* Define NDEBUG if you want to omit all assertions and thereby speed
   things up a little. */
#define NDEBUG  1

/* Shall the output be C code instead of a functional pseudo-code */
/* #define PRODUCE_C */

/* Shall the output be input to GraphViz's dot input file? */
/* #define PRODUCE_DOT */

/* A value larger or equal to `ceil(log_2(N))' */
#define BITS_IN_N   4

typedef unsigned int cost_t;
#define COST_INFINITE  0x3FF

/* Current cost at the root of the search tree. */
cost_t current_search_cost;

/* Parameters that control the format of output. */
#define VAR_NAME(i)   ("abcdefghijklmnopqrstuvxyz"[i])

/* Humm, should write a configure script, but too lazy for
   now.... :-S */
typedef unsigned int uint32_t;

/* End of configuration section. */

#define VERSION  "1.6"


#ifdef MAY_BE_UNDEFINED
#ifndef MAY_BE_EQUAL
#warning Can not handle partial orders without possible equality.
#warning Defining MAY_BE_EQUAL and trying in any case...
#define MAY_BE_EQUAL
#endif
#endif


#ifndef __GNUC__
#error Sorry, this code contains gcc-specific extensions.
#endif

#include <stdio.h>
#include <stdlib.h>
#include <malloc.h>
#include <assert.h>
#include <string.h>

#include <sys/times.h>
#include <unistd.h>
#include <math.h>

/* Requires GNU readline library. */
#include <readline/readline.h>


typedef enum {
  UNTESTED = 0,			/* Must be 0 */
  GREATER,
#ifdef MAY_BE_EQUAL
  EQUAL,
#endif
#ifdef MAY_BE_UNDEFINED
  GREATER_OR_EQUAL,
  INDIFFERENT,
  LESS_OR_EQUAL,
#endif
  LESS,
  N_ORDERS			/* Must be last */
} order_t;


static order_t opposite[N_ORDERS] = {
  UNTESTED,
  LESS,
#ifdef MAY_BE_EQUAL
  EQUAL,
#endif
#ifdef MAY_BE_UNDEFINED
  LESS_OR_EQUAL,
  INDIFFERENT,
  GREATER_OR_EQUAL,
#endif
  GREATER
};


static const char *order_name[N_ORDERS] = {
  "UNTESTED",
  "GREATER",
#ifdef MAY_BE_EQUAL
  "EQUAL",
#endif
#ifdef MAY_BE_UNDEFINED
  "GREATER_OR_EQUAL",
  "INDIFFERENT",
  "LESS_OR_EQUAL",
#endif
  "LESS"
};


static const char order_char[N_ORDERS] = {
  ' ',
  '>',
#ifdef MAY_BE_EQUAL
  '=',
#endif
#ifdef MAY_BE_UNDEFINED
  '»',
  '?',
  '«',
#endif
  '<'
};


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

#define N_PROBES  4
#define MAX_PRIORITY  0xFFFF
#define INITIAL_PRIORITY  0xFF
  unsigned int priority : 16;

  /* This is used to get some theoretically human-readable output. */
  unsigned int number : 16;

#ifdef MAY_BE_UNDEFINED
#define N_CMPS  ((((N * (N - 1)) / 2) + 15) / 10)
#else
#define N_CMPS  ((((N * (N - 1)) / 2) + 15) / 16)
#endif
  uint32_t order[N_CMPS];
} orderset_t;


/* Forward declaration. */
static cost_t search(orderset_t *, cost_t);


static int order_offset[N];

static void init_offsets(void)
{
  int s = -1, i;

  for (i = 0; i < N; i++) {
    order_offset[i] = s;
    s += N - i - 2;
  }
}


static inline void set_order(orderset_t *orderset, int a, int b, order_t order)
{
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
#ifdef MAY_BE_UNDEFINED
  shmt = (ix % 10) * 3;
  ix = ix / 10;
  mask = 0x7 << shmt;
#else
  shmt = (ix % 16) * 2;
  ix = ix / 16;
  mask = 0x3 << shmt;
#endif

  assert(ix < N_CMPS);
  assert(shmt < 32);

  orderset->order[ix] = (orderset->order[ix] & ~mask) | (order << shmt);
}

static inline order_t get_order(orderset_t *orderset, int a, int b)
{
  int n = orderset->n;
  order_t order;
  unsigned int ix, shmt;
  uint32_t mask;

  assert(a < n);
  assert(b < n);

  if (a == b) {
#ifdef MAY_BE_EQUAL
    return EQUAL;
#else
    assert(0);
    return N_ORDERS;
#endif
  }

  if (a < b)
    ix = order_offset[a] + b;
  else
    ix = order_offset[b] + a;
#ifdef MAY_BE_UNDEFINED
  shmt = (ix % 10) * 3;
  ix = ix / 10;
  mask = 0x7;
#else
  shmt = (ix % 16) * 2;
  ix = ix / 16;
  mask = 0x3;
#endif
  order = (orderset->order[ix] >> shmt) & mask;
  if (a > b)
    order = opposite[order];

  assert(order < N_ORDERS);

  return order;
}


static inline void copy_orderset(orderset_t *to, orderset_t *from)
{
  assert(to != NULL);
  assert(from != NULL);

  memcpy(to, from, sizeof(orderset_t));
  to->locked = 0;
}


static inline int orderset_is_equal(orderset_t *x, orderset_t *y)
{
  if (x == y)
    return 1;
  if (x->n != y->n
      || x->i != y->i
      || memcmp(x->order, y->order, N_CMPS * sizeof(x->order[0])))
    return 0;
  return 1;
}


static void inline clear_orderset(orderset_t *orderset)
{
  memset(orderset, 0, sizeof(*orderset));
}


static void print_orderset(orderset_t *orderset, 
			   unsigned char new_n,
			   unsigned char *new_a)
{
  order_t order;
  int a, b, c, n = orderset->n, is_first = 1;
  unsigned char new_a_reserve[N], old_a[N];

  if (new_a != NULL) {
    n = new_n;
    for (a = 0; a < n; a++)
      old_a[new_a[a]] = a;
  } else {
    for (a = 0; a < n; a++)
      old_a[a] = new_a_reserve[a] = a;
    new_a = new_a_reserve;
  }

#ifdef PRODUCE_DOT
  orderset_t os;

  if (new_a != new_a_reserve)
    /* PRODUCE_DOT not been implemented for new_a-indirection. */
    abort();
  clear_orderset(&os);
  copy_orderset(&os, orderset);
  for (a = 0; a < os.n; a++)
    for (b = 0; b < os.n; b++)
      if (a != b && get_order(&os, b, a) == LESS)
	for (c = 0; c < os.n; c++)
	  if (a != c && b != c && get_order(&os, c, b) == LESS)
	    set_order(&os, a, c, UNTESTED);
  printf("  subgraph cluster_find%d {\n", os.number);
  for (a = 0; a < n; a++)
    printf("    %c%d [label=\"%c\",peripheries=0];\n",
	   VAR_NAME(a), os.number, VAR_NAME(a));
  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++)
      switch (get_order(&os, a, b)) {
      case GREATER:
	printf("    %c%d -> %c%d\n",
	       VAR_NAME(a), os.number, VAR_NAME(b), os.number);
	break;
      case LESS:
	printf("    %c%d -> %c%d\n",
	       VAR_NAME(b), os.number, VAR_NAME(a), os.number);
	break;
      default:
	break;
      }
#if 0
  if (os.a != os.b)
    printf("    edge [style=dotted,arrowhead=none,arrowtail=none]\n"
	   "    %c%d -> %c%d [style=dotted];\n"
	   "    { rank=same; %c%d %c%d }\n",
	   VAR_NAME(os.a), os.number, VAR_NAME(os.b), os.number,
	   VAR_NAME(os.a), os.number, VAR_NAME(os.b), os.number);
#endif
  puts("  }");
#else
  printf("n=%d, i=%d", n, orderset->i + 1);
  if (orderset->solved)
    printf(", %c?%c", VAR_NAME(new_a[orderset->a]),
	   VAR_NAME(new_a[orderset->b]));
  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++) {
      order_t o;
      int is_transitive;

      if ((o = get_order(orderset, a, b)) == UNTESTED)
	continue;
      for (is_transitive = c = 0; c < n; c++)
	if (c != a
	    && c != b
	    && get_order(orderset, a, c) == o
	    && get_order(orderset, c, b) == o) {
	  is_transitive = 1;
	  break;
	}
      if (is_transitive)
	continue;
      if (is_first) {
	is_first = 0;
	printf(": ");
      } else
	printf(",");
      printf("%c%c%c", 
	     VAR_NAME(new_a[a]),
	     order_char[get_order(orderset, a, b)],
	     VAR_NAME(new_a[b]));
    }
  if (is_first)
    printf(": no comparisons done");
  putchar('\n');
#endif
}

/* Return non-zero on success, zero on failure. */
static int set_order_and_closure(orderset_t *orderset, int a, int b,
				 order_t order)
{
  int i, n = orderset->n;
  order_t old_order = get_order(orderset, a, b);

  switch (order) {
  case LESS:
    switch (old_order) {
    case LESS:
      break;
    case UNTESTED:
#ifdef MAY_BE_UNDEFINED
    case LESS_OR_EQUAL:
    case INDIFFERENT:
#endif
      set_order(orderset, a, b, LESS);
      for (i = 0; i < n; i++)
	if (i != a && i != b) {
	  switch (get_order(orderset, b, i)) {
	  case LESS:
#ifdef MAY_BE_EQUAL
	  case EQUAL:
#endif
	    if (!set_order_and_closure(orderset, a, i, LESS))
	      return 0;
	    break;
	  }
	  switch (get_order(orderset, a, i)) {
	  case GREATER:
#ifdef MAY_BE_EQUAL
	  case EQUAL:
#endif
	    if (!set_order_and_closure(orderset, b, i, GREATER))
	      return 0;
	    break;
	  }
	}
      break;
    default:
      return 0;
      break;
    }
    break;
    
#ifdef MAY_BE_EQUAL
  case EQUAL:
    switch (old_order) {
    case EQUAL:
      break;
    case UNTESTED:
#ifdef MAY_BE_UNDEFINED
    case GREATER_OR_EQUAL:
    case INDIFFERENT:
    case LESS_OR_EQUAL:
#endif
    make_a_b_equal:
      set_order(orderset, a, b, EQUAL);
      for (i = 0; i < n; i++)
	if (i != a && i != b) {
	  switch (get_order(orderset, b, i)) {
	  case EQUAL:
	    if (!set_order_and_closure(orderset, a, i, EQUAL))
	      return 0;
	    break;
	  case GREATER:
	    if (!set_order_and_closure(orderset, a, i, GREATER))
	      return 0;
	    break;
	  case LESS:
	    if (!set_order_and_closure(orderset, a, i, LESS))
	      return 0;
	    break;
	  }
	  switch (get_order(orderset, a, i)) {
	  case EQUAL:
	    if (!set_order_and_closure(orderset, b, i, EQUAL))
	      return 0;
	    break;
	  case GREATER:
	    if (!set_order_and_closure(orderset, b, i, GREATER))
	      return 0;
	    break;
	  case LESS:
	    if (!set_order_and_closure(orderset, b, i, LESS))
	      return 0;
	    break;
	  }
	}
      break;
    default:
      return 0;
      break;
    }
    break;
#endif

#ifdef MAY_BE_UNDEFINED
  case GREATER_OR_EQUAL:
    switch (old_order) {
    case GREATER:
    case GREATER_OR_EQUAL:
    case EQUAL:
      break;
    case LESS_OR_EQUAL:
      goto make_a_b_equal;
      break;
    case UNTESTED:
    case INDIFFERENT:
      set_order(orderset, a, b, GREATER_OR_EQUAL);
      for (i = 0; i < n; i++)
	if (i != a && i != b) {
	  switch (get_order(orderset, a, i)) {
	  case LESS:
	    if (!set_order_and_closure(orderset, b, i, LESS))
	      return 0;
	    break;
	  case LESS_OR_EQUAL:
	  case EQUAL:
	    if (!set_order_and_closure(orderset, b, i, LESS))
	      return 0;
	    break;
	  }
	  switch (get_order(orderset, b, i)) {
	  case GREATER:
	    if (!set_order_and_closure(orderset, a, i, GREATER))
	      return 0;
	    break;
	  case GREATER_OR_EQUAL:
	  case EQUAL:
	    if (!set_order_and_closure(orderset, a, i, GREATER_OR_EQUAL))
	      return 0;
	    break;
	  }
	}
      break;
    }
    break;
  case INDIFFERENT:
    if (old_order == UNTESTED) {
      set_order(orderset, a, b, INDIFFERENT);
      for (i = 0; i < n; i++)
	if (i != a && i != b) {
	  switch (get_order(orderset, a, i)) {
	  case GREATER:
	  case GREATER_OR_EQUAL:
	    if (!set_order_and_closure(orderset, b, i, GREATER_OR_EQUAL))
	      return 0;
	    break;
	  case EQUAL:
	  case INDIFFERENT:
	    if (!set_order_and_closure(orderset, b, i, INDIFFERENT))
	      return 0;
	    break;
	  case LESS:
	  case LESS_OR_EQUAL:
	    if (!set_order_and_closure(orderset, b, i, LESS_OR_EQUAL))
	      return 0;
	    break;
	  }
	  switch (get_order(orderset, b, i)) {
	  case GREATER:
	  case GREATER_OR_EQUAL:
	    if (!set_order_and_closure(orderset, a, i, GREATER_OR_EQUAL))
	      return 0;
	    break;
	  case EQUAL:
	  case INDIFFERENT:
	    if (!set_order_and_closure(orderset, a, i, INDIFFERENT))
	      return 0;
	    break;
	  case LESS:
	  case LESS_OR_EQUAL:
	    if (!set_order_and_closure(orderset, a, i, LESS_OR_EQUAL))
	      return 0;
	    break;
	  }
	}
    }
    break;
  case LESS_OR_EQUAL:
    switch (old_order) {
    case LESS:
    case LESS_OR_EQUAL:
    case EQUAL:
      break;
    case GREATER_OR_EQUAL:
      goto make_a_b_equal;
      break;
    case UNTESTED:
    case INDIFFERENT:
      set_order(orderset, a, b, LESS_OR_EQUAL);
      for (i = 0; i < n; i++)
	if (i != a && i != b) {
	  switch (get_order(orderset, a, i)) {
	  case GREATER:
	    if (!set_order_and_closure(orderset, b, i, GREATER))
	      return 0;
	    break;
	  case GREATER_OR_EQUAL:
	  case EQUAL:
	    if (!set_order_and_closure(orderset, b, i, GREATER))
	      return 0;
	    break;
	  }
	  switch (get_order(orderset, b, i)) {
	  case LESS:
	    if (!set_order_and_closure(orderset, a, i, LESS))
	      return 0;
	    break;
	  case LESS_OR_EQUAL:
	  case EQUAL:
	    if (!set_order_and_closure(orderset, a, i, LESS_OR_EQUAL))
	      return 0;
	    break;
	  }
	}
      break;
    }
    break;
#endif

  case GREATER:
    switch (old_order) {
    case GREATER:
      break;
    case UNTESTED:
#ifdef MAY_BE_UNDEFINED
    case GREATER_OR_EQUAL:
    case INDIFFERENT:
#endif
      set_order(orderset, a, b, GREATER);
      for (i = 0; i < n; i++)
	if (i != a && i != b) {
	  switch (get_order(orderset, b, i)) {
	  case GREATER:
#ifdef MAY_BE_EQUAL
	  case EQUAL:
#endif
	    if (!set_order_and_closure(orderset, a, i, GREATER))
	      return 0;
	    break;
	  }
	  switch (get_order(orderset, a, i)) {
	  case LESS:
#ifdef MAY_BE_EQUAL
	  case EQUAL:
#endif
	    if (!set_order_and_closure(orderset, b, i, LESS))
	      return 0;
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

/* The orderset must have been closured before calling the following two
   functions. */
static int orderset_is_complete(orderset_t *orderset)
{
  int a, b;

  /* No point in calling this for subordersets. */
  assert(orderset->n == N);
  assert(orderset->i == N);

  for (a = 0; a < N - 1; a++)
    for (b = a + 1; b < N; b++)
      if (get_order(orderset, a, b) == UNTESTED)
	return 0;
  return 1;
}

static int orderset_selects_ith(orderset_t *orderset, int i, 
				unsigned char *varp)
{
  int a, b, n = orderset->n, n_orders[N_ORDERS], j, slop;

  for (a = 0; a < N; a++) {
    for (j = 0; j < N_ORDERS; j++)
      n_orders[j] = 0;

    for (b = 0; b < N; b++)
      if (a != b)
	n_orders[get_order(orderset, b, a)]++;

    slop = 0;
#ifdef MAY_BE_EQUAL
    slop += n_orders[EQUAL];
#endif
#ifdef MAY_BE_UNDEFINED
    slop += n_orders[INDIFFERENT] + n_orders[GREATER_OR_EQUAL];
#endif
    if (n_orders[GREATER] + n_orders[UNTESTED] <= i
	&& i + n_orders[UNTESTED] <= n_orders[GREATER] + slop) {
      if (varp != NULL)
	*varp = a;
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
/* n= 0 */ { },
/* n= 1 */ { },
/* n= 2 */ {  1,  1 },
/* n= 3 */ {  2,  3,  2 },
/* n= 4 */ {  3,  4,  4,  3 },
/* n= 5 */ {  4,  6,  6,  6,  4 },
/* n= 6 */ {  5,  7,  8,  8,  7,  5 },
/* n= 7 */ {  6,  8, 10, 10, 10,  8,  6 },
/* n= 8 */ {  7,  9, 11, 12, 12, 11,  9,  7 },
/* n= 9 */ {  8, 11, 12, 14, 14, 14, 12, 11, 8 },
/* n=10 */ {  9, 12, 14, 15, 16, 16, 15, 14, 12,  9 },
/* n=11 */ { 10, 13, 15, 17, 18, 18, 18, 17, 15, 13, 10 },
/* n=12 */ { 11, 14, 17, 18, 19, 21, 21, 19, 18, 17, 14, 11 },
#if 0
/* n=13 */ { 12, 15, 18, 20, 21, 22, 22, 22, 21, 20, 18, 15, 12 },
                              /* ^^^^^^^^^^^ From theory */
/* n=14 */ { 13, 16, 19, 21, 22, 21, 22, 22, 21, 22, 21, 19, 16, 13 },
                          /* ^^^^^^^^^^^^^^^^^^^^^^ From theory */
/* n=15 */ { 14, 17, 20, 22, 22, 23, 23, 24, 23, 23, 22, 22, 20, 17, 14 },
                      /* ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ From theory */
#endif
};


orderset_t *hash_table = NULL;
uint32_t hash_table_size;

unsigned long long n_hits = 0, n_misses = 0;


static inline unsigned int cache_value(unsigned int n, unsigned int i)
{
#if 1
  int shmt = min_n_comparisons[n][i];

  if (shmt > current_search_cost)
    shmt = current_search_cost;

  return shmt;			/* 48.66s */
#endif

#if 0
  unsigned int mid = n / 2;

#if 1
  int shmt = min_n_comparisons[n][i];

  if (shmt > current_search_cost)
    shmt = current_search_cost;

  return shmt;			/* 48.66s */
  return shmt << 2;		/* 79.22s */
  
  shmt >>= 1;

  if (i <= mid)			/* =>>0: 92.56s, =>>1: 47.43s, =>>2: 67.07s,
				   =>>3: 82.86s, =>>4: 51.01s, =>>5: 53.59s */
    return n + (i << shmt);
  else
    return n + ((n - i - 1) << shmt);
#else
  if (i <= mid)			/* 63.52s */
    return i << (n + i / 4);
  else
    return (n - i - 1) << (n + (n - i - 1) / 4);
#endif
#endif

#if 0
  return n;			/* 53.70s */
  return 1;			/* 57.48s */
#endif
}


static inline uint32_t orderset_hash(orderset_t *orderset)
{
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


static orderset_t *probe_orderset_cache(orderset_t *orderset, 
					orderset_t **alts)
{
  orderset_t *alts_reserve[N_PROBES];
  orderset_t *p;
  int i, j;
  uint32_t h;
  /* Some random data to prevent possible infinite loops, taken from
     `cat /dev/random | od -x' */
#define N_RNDS  16
  static uint32_t rnd[N_RNDS] = {
    0x33432BD5, 0xD9DA2857, 0xC16CA349, 0xB48E786F,
    0xC280E04C, 0xB6419EF7, 0x35E68035, 0x7327028D,
    0x1618C293, 0x9E0A336E, 0x1709F35C, 0x5102EB06,
    0x2B02AD5B, 0x8E4D16D4, 0x2DEA4A71, 0x2A3DDDC0
  };
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

  if (alts == NULL)
    alts = alts_reserve;
  for (i = 0; i < N_PROBES; i++)
    alts[i] = NULL;
  for (i = 0; i < N_PROBES; i++) {
    p = &hash_table[h % hash_table_size];
    if (orderset_is_equal(p, orderset)) {
      /* Hit!  Increment priority and return. */
      unsigned long v = cache_value(p->n, p->i);
      if (p->priority <= MAX_PRIORITY - v)
	p->priority += v;
      n_hits++;
      return p;
    } else if (!p->locked) {
      for (j = 0; j < i; j++)
	if (alts[j] == p) {
	  /* We don't want to identical pointers in `alts'.  Stir up
	     `h' and pick another slot in the hash table. */
	  h ^= rnd[rnd_ix++];
	  if (rnd_ix == N_RNDS)
	    rnd_ix = 0;
	  i--;
	  goto next_probe;
	}
      alts[i] = p;
    }
    h += h << 3;
    h ^= h >> 11;
    h += h << 15;
  next_probe:
    ;
  }

  /* Miss. */
  n_misses++;
  return NULL;
}

static inline void store_orderset_cache(orderset_t *orderset,
					orderset_t **alts)
{
  int i, j;

  for (i = j = 0; i < N_PROBES; i++) {
    if (alts[i] == NULL) {
      if (i == j)
	j++;
      continue;
    }
    if (i == j || alts[j]->priority > alts[i]->priority)
      j = i;
  }
  if (j < N_PROBES && alts[j] != NULL) {
    copy_orderset(alts[j], orderset);
    alts[j]->priority = cache_value(orderset->n, orderset->i);
  }

  /* Decrease weight of all other alternatives.
   */
  for (i = 0; i < N_PROBES; i++)
    if (i != j && alts[i] != NULL && alts[i]->priority > 1)
      alts[i]->priority--;
}

static void report_orderset_cache(void)
{
  unsigned long long priority_sum = 0, cost_sum = 0;
  unsigned int i, n_solved = 0, n_unsolved = 0;
  orderset_t *p;

  for (i = 0; i < hash_table_size; i++) {
    p = &hash_table[i];
    if (p->cost == 0)
      /* Empty. */
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
  printf("# avg_priority = %.3f\n", 
	 priority_sum / (double) (n_solved + n_unsolved));
  printf("# avg_cost = %.3f\n", 
	 cost_sum / (double) (n_solved + n_unsolved));
}


/* Perform various simplifications and canonifications in the
   orderset, and detects whether the orderset has become solved. */
static inline void canonify_orderset(orderset_t *orderset, 
				     unsigned char *new_a)
{
  unsigned char dropped[N], new_a_reserve[N], n_orders[N][N_ORDERS];
  unsigned int lessness[N], n_topo_iterations;
  order_t order;
  uint32_t topo_hash_old[N], topo_hash_new[N], h;
  int n = orderset->n, new_n, n_less_dropped, a, b, c, j;
  int have_dropped, n_cmps;
  orderset_t new_orderset;

  if (new_a == NULL)
    new_a = new_a_reserve;

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
    n = new_n;			/* `new_n' is a cumbersome name... */
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
    topo_hash_old[new_a[a]]
      = lessness[new_a[a]]
      = N*N
        - (n_orders[new_a[a]][GREATER] << BITS_IN_N)
        - n_orders[new_a[a]][LESS];
  
  for (n_topo_iterations = 0;
       n_topo_iterations < N_TOPO_ITERATIONS;
       n_topo_iterations++) {

    /* Compute new hash values based on the old ones. */
    for (a = 0; a < n; a++) {
      h = topo_hash_old[new_a[a]];
      h += h << 10;
      for (b = 0; b < n; b++)
	if (a != b
	    && (order = get_order(orderset, new_a[a], new_a[b])) != UNTESTED)
	  h += (topo_hash_old[new_a[b]] << 4) + order;
	  /* NOTE: This hash function must be commutative w.r.t. all
	     other compared variables `new_a[b]'.  Therefore we can
	     *not* issue the typical `h += h << 10; h ^= h >> 7;' here. */
      topo_hash_new[new_a[a]] = h;
    }

#if 0
    /* This was useful for testing. */
    for (a = 0; a < n - 1; a++)
      for (b = a + 1; b < n; b++)
	if (topo_hash_old[new_a[a]] != topo_hash_old[new_a[b]]
	    && topo_hash_new[new_a[a]] == topo_hash_new[new_a[b]]) {
	  /* print_orderset uses new_a differently now. */
	  print_orderset(orderset, n, new_a);
	  printf("n_topo_iterations = %d:\nold:\n", n_topo_iterations);
	  for (a = 0; a < n; a++)
	    printf("%c:%d-%08X ", VAR_NAME(new_a[a]), 
		   lessness[new_a[a]], topo_hash_old[new_a[a]]);
	  printf("\nnew:\n");
	  for (a = 0; a < n; a++)
	    printf("%c:%d-%08X ", VAR_NAME(new_a[a]),
		   lessness[new_a[a]], topo_hash_new[new_a[a]]);
	  printf("\n");
	  goto foo;
	}
  foo:
#endif

    /* Propagate new hash values to become the old ones in the next
       iteration. */
    for (a = 0; a < n; a++)
      topo_hash_old[new_a[a]] = topo_hash_new[new_a[a]];
  }

  /* A simple O(n^2) sort based on the lessness and hash values. */
  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++)
      if (lessness[new_a[a]] < lessness[new_a[b]]
	  || (lessness[new_a[a]] == lessness[new_a[b]]
	      && topo_hash_old[new_a[a]] < topo_hash_old[new_a[b]])) {
	j = new_a[a];
	new_a[a] = new_a[b];
	new_a[b] = j;
      }

  /* The final pass. */
  for (a = 1; a < n; a++)
    if (lessness[new_a[a]] == lessness[new_a[a - 1]]
	&& topo_hash_old[new_a[a]] == topo_hash_old[new_a[a - 1]]) {
      /* Skip over the equivalent variables. */
      for (a++; 
	   a < n
	     && lessness[new_a[a]] == lessness[new_a[a - 1]]
	     && topo_hash_old[new_a[a]] == topo_hash_old[new_a[a - 1]];
	   a++)
	;
      if (a == n)
	continue;
      for (n_topo_iterations = 0;
	   n_topo_iterations < N_TOPO_ITERATIONS2;
	   n_topo_iterations++) {
	/* Recompute the hash values. */
	for (c = a; c < n; c++) {
	  h = topo_hash_old[new_a[c]];
	  h += h << 10;
	  for (b = 0; b < a; b++)
	    if ((order = get_order(orderset, new_a[c], new_a[b]))
		!= UNTESTED)
	      h += (topo_hash_old[new_a[b]] << (4 + BITS_IN_N))
		- (order << BITS_IN_N) + b;
	  topo_hash_new[new_a[c]] = h;
	}
	/* Propagate hash values in case we should hash again. */	
	for (c = a; c < n; c++)
	  topo_hash_old[new_a[c]] = topo_hash_new[new_a[c]];
      }
      /* Re-sort the variables below the equivalent ones we just
	 skipped over. */
      for (c = a; c < n - 1; c++)
	for (b = c + 1; b < n; b++)
	  if (lessness[new_a[c]] < lessness[new_a[b]]
	      || (lessness[new_a[c]] == lessness[new_a[b]]
		  && topo_hash_old[new_a[c]] < topo_hash_old[new_a[b]])) {
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
    for (b = a + 1; b < n; b++)
      set_order(&new_orderset, a, b,
		get_order(orderset, new_a[a], new_a[b]));
  copy_orderset(orderset, &new_orderset);

  /* Detect if the problem became solved, and feed the killer
     heuristic with a guesstimate of how difficult a problem we have
     left. */
  if (n == 1)
    orderset->solved = 1;
}


static unsigned int inline highest_bit(uint32_t x)
{
  unsigned int i;

  if (x-- == 0)
    return 0;
  for (i = 0; x != 0; i++)
    x >>= 1;
  return i;
}


typedef int n_orders_t[N][N_ORDERS];

typedef enum {
  SOLVED, IMPOSSIBLE, UNCLEAR
} solvability_t;

/* Return non-zero if the given orderset might be solvable in
   `cost_limit' comparisons.  Returns zero only if it is completely
   certain that a solution can not be found in the given number of
   comparisons. */

static inline int estimate_hardness(orderset_t *orderset,
				    int *hardness,
				    int cost_limit,
				    int changed)
{
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
      if (d < 0) d = - d;
      *hardness -= d + 2 * u;
    }
  }


  if (orderset->i == 0 || orderset->i == n - 1)
    return cost_limit >= n - 1;
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
#if 1
    int n_cmps = min_n_comparisons[n][orderset->i];

    /* Subtract from `n_cmps' every order which is not deducible from
       some other order.  Try first the most easily computed ones. */
    for (a = 0; a < n; a++)
      if (n_orders[a][LESS] == 1) {
	n_cmps--;
	if (n_cmps <= cost_limit)
	  goto out;
      }
    for (a = 0; a < n - 1; a++)
      if (n_orders[a][LESS] >= 2)
	for (b = a + 1; b < n; b++) {
	  if (get_order(orderset, b, a) == LESS)
	    if (n_orders[b][GREATER] == 1) {
	      n_cmps--;
	      if (n_cmps <= cost_limit)
		goto out;
	    } else {
	      for (c = 0; c < n; c++)
		if (a != c
		    && b != c
		    && get_order(orderset, b, c) == LESS
		    && get_order(orderset, c, a) == LESS)
		  goto next_b;
	      n_cmps--;
	      if (n_cmps <= cost_limit)
		goto out;
	    next_b:
	      ;
	    }
	}
  out:
    if (n_cmps > cost_limit)
      return 0;
#endif

#if 0
    /* XXX THIS IS INCORRECT! */

    /* Without this heuristic: 697s, with it 330s */
    for (a = 0; a < n; a++) {
      int n_gt, n_lt, n_both;

      if (n - n_orders[a][LESS] - n_orders[a][GREATER] - 1 <= cost_limit)
	/* A quick pre-test. */
	continue;

      n_gt = n_lt = n_both = 0;
      for (b = 0; b < n; b++)
	if (a != b
	    && get_order(orderset, a, b) == UNTESTED) {
	  if (n_orders[b][LESS] == 0)
	    if (n_orders[b][GREATER] == 0)
	      n_both++;
	    else
	      n_gt++;
	  else if (n_orders[b][GREATER] == 0)
	    n_lt++;
	}
      while (n_lt < orderset->i - n_orders[a][LESS] && n_both > 0) {
	n_lt++;
	n_both--;
      }
      while (n_gt < n - orderset->i - 1 - n_orders[a][GREATER] && n_both > 0) {
	n_gt++;
	n_both--;
      }
      if (n_lt + n_gt > cost_limit)
	return 0;
    }
#endif

#if 0
    if (n >= 5 && n <= 8 && changed) {
      copy_orderset(&os, orderset);
      
      /* Remove from `os' all comparisons whose utility is one (or
	 more). */
      for (n_cmps = a = 0; a < n; a++)
	if (n_orders[a][LESS] == 1)
	  for (b = 0; b < n; b++)
	    if (a != b
		&& n_orders[b][GREATER] == 1
		&& get_order(&os, a, b) == GREATER) {
	      set_order(&os, a, b, UNTESTED);
	      n_cmps++;
	      break;
	    }
      canonify_orderset(&os, NULL);
      cost = search(&os, cost_limit + n_cmps);
      return cost <= cost_limit + n_cmps;
    }
#endif

#if 0
    if (n >= 8 && changed) {
      int c;

      copy_orderset(&os, orderset);
      
      /* Add to `os' comparisons that are most probably pretty
	 useless. */
      for (a = 0; a < n; a++)
	if (n_orders[a][LESS] == 0
	    && n_orders[a][GREATER] >= 2) {
	  for (c = 0; c < n; c++)
	    if (a != c 
		&& n_orders[c][LESS] >= 1
		&& n_orders[c][GREATER] >= 1
		&& get_order(orderset, a, c) == LESS) {
	      for (b = 0; b < n; b++)
		if (a != b
		    && n_orders[b][GREATER] == 0
		    && n_orders[b][LESS] >= 2
		    && get_order(orderset, a, b) == UNTESTED) {
		  for (c = 0; c < n; c++)
		    if (b != c
			&& n_orders[c][GREATER] >= 1
			&& n_orders[c][LESS] >= 1
			&& get_order(orderset, b, c) == GREATER) {
		      set_order(&os, a, b, LESS);
		      goto gout1;
		    }
		gout1:
		  ;
		}
	      goto gout2;
	    }
	gout2:
	  ;
	}
	      
      canonify_orderset(&os, NULL);
      cost = search(&os, cost_limit);
      if (cost > cost_limit)
	return 0;
    }
#endif
  }

  return 1;
}


typedef struct {
  unsigned long hash : 28;
  unsigned int n_hits : 4;
} fw_cache_entry_t;

#define N_FW_CACHE_ENTRIES  (1024*1024)

static fw_cache_entry_t fw_cache[N_FW_CACHE_ENTRIES];


static inline solvability_t fw_search(orderset_t *orderset,
				      n_orders_t n_orders,
				      int initial_a, int initial_b,
				      int cost_limit,
				      int depth)
{
  int a, b, d, n = orderset->n;
  orderset_t os, *cached_os;
  solvability_t s;

  copy_orderset(&os, orderset);
  if (initial_a != 0 || initial_b != 0) {
    canonify_orderset(&os, NULL);
    if (!estimate_hardness(&os, NULL, cost_limit, 0))
      return IMPOSSIBLE;
    cached_os = probe_orderset_cache(&os, NULL);
    if (cached_os != NULL)
      if (cached_os->solved)
	if (cached_os->cost <= cost_limit)
	  return SOLVED;
	else
	  return IMPOSSIBLE;
      else
	if (cached_os->cost >= cost_limit)
	  return IMPOSSIBLE;
	else
	  goto unclear;
  }

  /* Experiments showed there's little benefit from depth-limiting the
     loop below. */
  for (a = initial_a; a < n; a++)
    if (n_orders[a][LESS] == 0 && n_orders[a][GREATER] >= 2)
      for (b = (a == initial_a ? initial_b : 0); b < n; b++)
	if (a != b
	    && n_orders[b][GREATER] == 0
	    && n_orders[b][LESS] >= 2
	    /* && (d = n_orders[a][GREATER] + n_orders[b][LESS]) >= 5 */
	    && get_order(orderset, a, b) == UNTESTED) {
	  set_order(orderset, a, b, LESS);
	  s = fw_search(orderset, n_orders, a, b + 1, cost_limit, depth + 1);
	  set_order(orderset, a, b, UNTESTED);
	  if (s == IMPOSSIBLE)
	    return IMPOSSIBLE;
	}

 unclear:
  if (initial_a != 0 || initial_b != 0) {
    uint32_t h = orderset_hash(&os);
    uint32_t i = h % N_FW_CACHE_ENTRIES;

    if (fw_cache[i].hash == (h >> 4)) {
      if (fw_cache[i].n_hits >= 6 && depth <= 4) {
	int cost = search(&os, cost_limit);
	if (cost > cost_limit)
	  return IMPOSSIBLE;
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

static cost_t search(orderset_t *parent, cost_t cost_limit)
{
  orderset_t *alts[N_PROBES], *cached_parent, child[N_ORDERS];
  cost_t cost, child_cost;
  order_t order, killer_order;
  int a, b, c, d, hardness, worst_hardness, n, new_n, delta = 0;

  if (parent->solved)
    return parent->cost;
  if (cost_limit <= 0)
    return COST_INFINITE;
  cached_parent = probe_orderset_cache(parent, alts);
  if (cached_parent != NULL) {
    if (cached_parent->solved)
      return cached_parent->cost;
    if (cached_parent->cost >= cost_limit)
      return COST_INFINITE;
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
    if (fw_search(parent, n_orders, 0, 0, cost_limit, 0) == IMPOSSIBLE)
      return COST_INFINITE;
  }

  if (cached_parent != NULL)
    parent = cached_parent;

#if 1
  if (nth_orderset++ % 1024 == 0) {
    printf("limit=%d, ", current_search_cost);
    print_orderset(parent, 0, NULL);
  }
  if (nth_orderset % 65536 == 0)
    report_orderset_cache();
#endif

  for (a = 0; a < n - 1; a++)
    for (b = a + 1; b < n; b++)
      if (get_order(parent, a, b) == UNTESTED) {

	worst_hardness = -1000000;
	killer_order = GREATER;
	for (order = GREATER; order <= LESS; order++) {
	  copy_orderset(&child[order], parent);
	  set_order_and_closure(&child[order], a, b, order);
	  canonify_orderset(&child[order], NULL);
	  if (!estimate_hardness(&child[order],
				 &hardness,
				 cost_limit - 1 - delta,
				 n != child[order].n))
	    goto next_comparison;
	  {
	    orderset_t *cached_child = 
	      probe_orderset_cache(&child[order], NULL);
	    if (cached_child != NULL) {
	      if (!cached_child->solved
		  && cached_child->cost >= cost_limit - 1 - delta)
		goto next_comparison;
	    }
	  }
#if 1
	  if (hardness > worst_hardness) {
	    worst_hardness = hardness;
	    killer_order = order;
	  }
#endif
	}

	cost = 0;
	for (order = UNTESTED; order <= LESS; order++) {
	  if (order == UNTESTED)
	    child_cost = search(&child[killer_order], cost_limit - 1 - delta);
	  else if (order == killer_order)
	    continue;
	  else
	    child_cost = search(&child[order], cost_limit - 1 - delta);
	  if (child_cost > cost_limit - 1 - delta)
	    /* This test is equivalent to an alpha-beta cutoff. */
	    goto next_comparison;
	  child_cost++;
	  if (child_cost > cost)
	    cost = child_cost;
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
      next_comparison:
	;
      }

  n_nodes++;

  if (!parent->solved)
    parent->cost = cost_limit;
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

static void research(orderset_t *parent, int cost_limit)
{
  orderset_t *alts[N_PROBES], *cached_parent, child, *cached_child;
  unsigned char new_a[N];
  cost_t cost;
  order_t order;
  int a, b, i, n, greater_done = 1, less_done = 1;

  n = parent->n;
  cached_parent = probe_orderset_cache(parent, alts);
  if (cached_parent == NULL) {
    cost = search(parent, cost_limit);
    cached_parent = probe_orderset_cache(parent, alts);
  }
  if (cached_parent->number == 0)
    cached_parent->number = number++;
  a = cached_parent->a;
  b = cached_parent->b;

#ifdef PRODUCE_DOT
  print_orderset(cached_parent, 0, NULL);
#else
#ifdef PRODUCE_C
  printf("/*\n");
#else
  printf("# ");
#endif
  print_orderset(cached_parent, 0, NULL);
#ifdef PRODUCE_C
  printf(" */\n");
  printf("int find_%d(", cached_parent->number);
#else
  printf("find_%d(", cached_parent->number);
#endif
  for (i = 0; i < n; i++) {
#ifdef PRODUCE_C
    printf("int %c", VAR_NAME(i));
    if (i < n-1)
      printf(", ");
#else
    putchar(VAR_NAME(i));
    if (i < n-1)
      putchar(',');
#endif
  }
#ifdef PRODUCE_C
  printf(")\n{\n  return %c > %c\n    ? ", VAR_NAME(a), VAR_NAME(b));
#else
  printf(") =\n  if %c > %c\n  then ", VAR_NAME(a), VAR_NAME(b));
#endif
#endif /* !PRODUCE_DOT */

  copy_orderset(&child, parent);
  set_order_and_closure(&child, a, b, GREATER);
  canonify_orderset(&child, new_a);
  if (child.solved)
#ifdef PRODUCE_DOT
    ;
#else
    putchar(VAR_NAME(new_a[0]));
#endif
  else {
    search(&child, cost_limit - 1);
    cached_child = probe_orderset_cache(&child, alts);
    if (cached_child->number == 0) {
      greater_done = 0;
      cached_child->number = number++;
    }
#ifdef PRODUCE_DOT
    printf("  %c%d -> %c%d [ltail = cluster_find%d, lhead = cluster_find%d, "
	   "label=\"%c>%c\"];\n",
	   VAR_NAME(0), cached_parent->number,
	   VAR_NAME(0), cached_child->number,
	   cached_parent->number,
	   cached_child->number,
	   VAR_NAME(a), VAR_NAME(b));
#else
    n = child.n;
    printf("find_%d(", cached_child->number);
    for (i = 0; i < n; i++) {
      putchar(VAR_NAME(new_a[i]));
      if (i < n-1)
	putchar(',');
    }
    putchar(')');
#endif /* !PRODUCE_DOT */
  }
#ifndef PRODUCE_DOT
#ifdef PRODUCE_C
  printf("\n    : ");
#else
  printf("\n  else ");
#endif
#endif

  copy_orderset(&child, parent);
  set_order_and_closure(&child, a, b, LESS);
  canonify_orderset(&child, new_a);
  if (child.solved)
#ifdef PRODUCE_DOT
    ;
#else
    putchar(VAR_NAME(new_a[0]));
#endif
  else {
    search(&child, cost_limit - 1);
    cached_child = probe_orderset_cache(&child, alts);
    if (cached_child->number == 0) {
      less_done = 0;
      cached_child->number = number++;
    }
#ifdef PRODUCE_DOT
    printf("  %c%d -> %c%d [ltail = cluster_find%d, lhead = cluster_find%d, "
	   "label=\"%c<%c\"];\n",
	   VAR_NAME(0), cached_parent->number,
	   VAR_NAME(0), cached_child->number,
	   cached_parent->number, cached_child->number,
	   VAR_NAME(a), VAR_NAME(b));
#else
    n = child.n;
    printf("find_%d(", cached_child->number);
    for (i = 0; i < n; i++) {
      putchar(VAR_NAME(new_a[i]));
      if (i < n-1)
	putchar(',');
    }
    putchar(')');
#endif /* !PRODUCE_DOT */
  }
#ifndef PRODUCE_DOT
#ifdef PRODUCE_C
  printf(";\n}\n\n");
#else
  puts("");
#endif
#endif /* !PRODUCE_DOT */

  if (!less_done) {
    copy_orderset(&child, parent);
    set_order_and_closure(&child, a, b, LESS);
    canonify_orderset(&child, NULL);
    research(&child, cost_limit - 1);
  }

  if (!greater_done) {
    copy_orderset(&child, parent);
    set_order_and_closure(&child, a, b, GREATER);
    canonify_orderset(&child, NULL);
    research(&child, cost_limit - 1);
  }
}


/* Returns wall clock time in seconds since the beginning of the
   program.  Typical use involves two calls; one before and one after
   the benchmark, and the consumed time is the difference of these two
   times. */

static double give_time(void)
{
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
  return ticks / (double) ticks_per_second;
}


/* An interactive decision tree studying routine.  Returns non-zero if
   full exit not requested. */

static int study(orderset_t *orderset, unsigned char *new_a)
{
  orderset_t *cached_orderset, os;
  char *cmd, old_a[N], newing_a[N], newer_a[N];
  cost_t cost, max_cost;
  int a, b;
  order_t order;

  for (a = 0; a < N; a++) {
    /* printf("new_a[%d] = %d\n", a, new_a[a]); */
    old_a[new_a[a]] = a;
  }

  while (1) {
    cached_orderset = probe_orderset_cache(orderset, NULL);
    if (cached_orderset != NULL)
      orderset = cached_orderset;
    print_orderset(orderset, N, new_a);
    fflush(stdout);
    cmd = readline("# ");
    
    if (cmd == NULL)
      return 0;
    if (cmd[0] == '-' && cmd[1] == '\0')
      /* Go one step up towards to root of the decision tree. */
      return 1;
    else if (cmd[0] == '!' && cmd[1] == '\0') {
      printf("   ");
      for (b = 1; b < N; b++)
	printf("  %c", VAR_NAME(b));
      for (a = 0; a < N - 1; a++) {
	puts("");
	if (a > 0)
	  printf("%*c", 3*a, ' ');
	printf("%c ?", VAR_NAME(a));
	for (b = a + 1; b < N; b++)
	  if (get_order(orderset, old_a[a], old_a[b]) != UNTESTED)
	    printf("   ");
	  else {
	    fflush(stdout);
	    current_search_cost = 0;
	    do {
	      copy_orderset(&os, orderset);
	      set_order_and_closure(&os, old_a[a], old_a[b], LESS);
	      canonify_orderset(&os, NULL);
	      cost = search(&os, current_search_cost);
	    } while (cost > current_search_cost++);
	    max_cost = cost;
	    current_search_cost = 0;
	    do {
	      copy_orderset(&os, orderset);
	      set_order_and_closure(&os, old_a[a], old_a[b], GREATER);
	      canonify_orderset(&os, NULL);
	      cost = search(&os, current_search_cost);
	    } while (cost > current_search_cost++);
	    if (cost > max_cost)
	      max_cost = cost;
	    printf("%3d", max_cost);
	  }
      }
      puts("");
    } else if (cmd[0] == '?' && cmd[1] == '\0') {
      printf("   ");
      for (b = 0; b < N; b++)
	printf("  %c", VAR_NAME(b));
      for (a = 0; a < N; a++) {
	printf("\n%c <", VAR_NAME(a));
	for (b = 0; b < N; b++)
	  if (get_order(orderset, old_a[a], old_a[b]) != UNTESTED)
	    printf("   ");
	  else {
	    fflush(stdout);
	    current_search_cost = 0;
	    do {
	      copy_orderset(&os, orderset);
	      set_order_and_closure(&os, old_a[a], old_a[b], LESS);
	      canonify_orderset(&os, NULL);
	      cost = search(&os, current_search_cost);
	    } while (cost > current_search_cost++);
	    printf("%3d", cost);
	  }
      }
      puts("");
    } else if (cmd[0] >= 'a' 
	       && cmd[0] < 'a' + N 
	       && (cmd[1] == '<' || cmd[1] == '>')
	       && cmd[2] >= 'a'
	       && cmd[2] < 'a' + N
	       && cmd[3] == '\0') {
      a = cmd[0] - 'a';
      b = cmd[2] - 'a';
      order = cmd[1] == '<' ? LESS : GREATER;
      if (get_order(orderset, old_a[a], old_a[b]) != UNTESTED)
	printf("The relative order of %c and %c is already known.\n",
	       VAR_NAME(a), VAR_NAME(b));
      else {
	copy_orderset(&os, orderset);
	set_order_and_closure(&os, old_a[a], old_a[b], order);
	canonify_orderset(&os, newing_a);
	for (a = 0; a < N; a++) {
	  newer_a[a] = new_a[newing_a[a]];
	  /* printf("a=%d, new=%d, newing=%d, newer=%d\n",
	     a, new_a[a], newing_a[a], newer_a[a]); */
	}
	if (!study(&os, newer_a))
	  return 0;
      }
    } else
      printf("Unrecognized command.\n");
  }
}


int main(int argc, char **argv)
{
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
#ifdef ASSUME_INITIAL_PAIRWISE
  for (a = 0; a < N - 1; a += 2)
    set_order(&initial, a, a + 1, GREATER);
#endif

#ifdef PRINT_COST
  set_order_and_closure(&initial, 0, 1, GREATER);
  set_order_and_closure(&initial, 0, 2, GREATER);
  set_order_and_closure(&initial, 2, 3, GREATER);
  set_order_and_closure(&initial, 3, 4, GREATER);
#endif

  canonify_orderset(&initial, NULL);
  if (initial.solved)
    found_cost = 0;
  else {
    current_search_cost = 1;
    do {
#ifndef PRINT_COST
      fprintf(stderr, "searching V(%d,%d) with maximum cost %d\n", 
	      N, I, current_search_cost);
#endif
      /* n_nodes = n_hits = n_misses = 0; */
      found_cost = search(&initial, current_search_cost);
    } while (found_cost > current_search_cost++);
  }

#ifdef PRINT_COST
  printf("%3d", min_n_comparisons[N][I - 1] - found_cost);
  return 0;
#endif

  t_end = give_time();
  fprintf(stderr, "solved V(%d,%d) = %d\n", N, I, found_cost);

#ifdef PRODUCE_DOT
  printf("digraph V_%d_%d {\n", N, I);
  puts("  compound=true;");
  puts("  size=\"8.4,11.9\";");
#else
#ifdef PRODUCE_C
  puts("/*");
#endif
#ifdef ASSUME_INITIAL_PAIRWISE
  puts("# NOTE: This code has been generated using the assumption that we");
  puts("# first compare pairwise all variables, which in this case requires");
  printf("# performing %d comparisons.  Gasarch et al. conjecture that this\n",
	 N / 2);
  puts("# should not lead to suboptimal selection algorithms.  In this");
  printf("# particular case (N=%d, I=%d) the found algorithm's complexity\n",
	 N, I);
  puts("# coincides with the best known theoretical result, and we can hence");
  puts("# deduce that this algorithm is optimal.\n");
#endif

  printf("# cost = %d\n", (int) found_cost);
  printf("# n_nodes = %llu\n", n_nodes);
  printf("# version = " VERSION "\n");
  printf("# time = %.3f\n", t_end - t_start);
  report_orderset_cache();
#ifdef PRODUCE_C
  puts("*/");
#else
  puts("");
#endif
#endif /* PRODUCE_DOT */
  fflush(stdout);

  research(&initial, found_cost);

#ifdef PRODUCE_DOT
  puts("}");
#endif

#ifdef STUDY
  for (a = 0; a < N; a++)
    new_a[a] = a;
  study(&initial, new_a);
#endif

  return 0;
}
