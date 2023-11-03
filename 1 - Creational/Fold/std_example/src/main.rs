
/// A biblioteca padrão do Rust também implementa o modo de iteração Fold.

fn main() {

    // Soma todos os elementos do array.
    let a = [1, 2, 3];
    let sum = a.iter().fold(0, |acc, x| acc + x);
    println!("num: {}", sum);

    // O mesmo que acima, mas usando rfold.
    let sum = a.iter().rfold(0, |acc, &x| acc + x);
    println!("num: {}", sum);

    // Gera lista com elementos concatenados alinhados à esquerda.
    let numbers = [1, 2, 3, 4, 5];
    let zero = "0".to_string();

    let result = numbers.iter().fold(zero, |acc, &x| {
        format!("({acc} + {x})")
    });

    println!("result: {}", result);

    // O mesmo que o acima, mas alinhados à direita.
    let zero = "0".to_string();
    let result = numbers.iter().rfold(zero, |acc, &x| {
        format!("({x} + {acc})")
    });

    println!("result: {}", result);
}


