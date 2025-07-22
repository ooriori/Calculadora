fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn restar(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let a = 10;
    let b = 4;

    let resultado_suma = sumar(a, b);
    let resultado_resta = restar(a, b);

    println!("La suma de {} + {} es: {}", a, b, resultado_suma);
    println!("La resta de {} - {} es: {}", a, b, resultado_resta);
}
