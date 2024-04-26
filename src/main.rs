static mut STATIC_VARIAVEL: i32 = 15;
fn main()
{
    // exemplo 1
    let a: i32 = 10;
    {
        println!("O valor de a é {}", a);
        let b: i32 = 20;
        println!("Soma de {} e {} é {}", a, b, a + b);

        let a: f32 = 20.302;
        println!("O valor de a é {}", a);
    }
    println!("O valor de a é {}", a);

    // exemplo 2
    // println!("O valor da STATIC_VARIAVEL é {}", STATIC_VARIAVEL); // não é seguro utilizar desta forma.

    unsafe{
        println!("O valor da STATIC_VARIAVEL é {}", STATIC_VARIAVEL);
    }
}
