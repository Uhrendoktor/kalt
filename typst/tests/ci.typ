// Machine-readable test suite. The same operation cases are used by the
// visual suite; utils.typ switches their rendering off in CI mode.

#import "utils.typ": emit-test-report

#include "ops/abs.typ";
#include "ops/add.typ";
#include "ops/binom.typ";
#include "ops/ceil.typ";
#include "ops/cross.typ";
#include "ops/conjugate.typ";
#include "ops/dot.typ";
#include "ops/sub.typ";
#include "ops/mul.typ";
#include "ops/div.typ";
#include "ops/floor.typ";
#include "ops/factorial.typ";
#include "ops/index.typ";
#include "ops/pow.typ";
#include "ops/root.typ";
#include "ops/sign.typ";
#include "ops/ln.typ";
#include "ops/log.typ";
#include "ops/re_im.typ";

#emit-test-report()
