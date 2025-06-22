mod mod1;
mod mod2;
mod subfolder {
    pub mod mod3;
}

fn main() {
    let num1 = mod1::fn1();
    let num2 = mod2::fn2();
    let num3 = subfolder::mod3::fn3();


    println!("{}", num1 + num2 + num3);
}
