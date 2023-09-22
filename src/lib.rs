use a::A;
use a1::A1;
use a2::A2;

fn _test(x: impl A) {
    x.a_ref();
    let mut x = x.a_move();
    x.a_mut();
}

fn _test1(x: impl A1) {
    _test(x)
}

fn _test2(x: impl A2) {
    _test(x)
}

fn _testdyn(x: &mut (impl ?Sized + A)) {
    x.a_ref();
    x.a_mut();
}

fn _testdyn1(x: &mut (impl ?Sized + A1)) {
    _testdyn(x)
}

fn _testdyn2(x: &mut (impl ?Sized + A2)) {
    _testdyn(x)
}
