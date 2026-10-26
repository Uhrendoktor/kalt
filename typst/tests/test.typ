// Visual and CI test suite for kalt

#import "utils.typ": emit-test-report

#set page(margin: 14pt)

#include "ops/abs.typ";
#include "ops/add.typ";
#include "ops/binom.typ";
#include "ops/ceil.typ";
#include "ops/cross.typ";
#include "ops/conjugate.typ";
#include "ops/dot.typ";
#include "ops/sub.typ";
#include "ops/mul.typ";
#include "ops/modulo.typ";
#include "ops/sin.typ";
#include "ops/asin.typ";
#include "ops/cos.typ";
#include "ops/acos.typ";
#include "ops/tan.typ";
#include "ops/atan.typ";
#include "ops/sinh.typ";
#include "ops/cosh.typ";
#include "ops/tanh.typ";
#include "helpers.typ";
#include "ops/div.typ";
#include "ops/floor.typ";
#include "ops/factorial.typ";
#include "ops/index.typ";
#include "ops/pow.typ";
#include "ops/root.typ";
#include "ops/sign.typ"
#include "ops/ln.typ"
#include "ops/log.typ"
#include "ops/re_im.typ"
#include "errors.typ"

#emit-test-report()
