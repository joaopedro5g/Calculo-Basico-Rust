use std::{io::stdin, thread::sleep, time::Duration};

fn get_line_cmd() -> String {
  let mut calc = String::new();
  stdin().read_line(&mut calc).expect("Erro ao ler o tipo de calculo a ser usado");
  calc
}

fn computed(text: &str) -> i64 {
  let mut val = String::new();
  println!("{}", text);
  stdin().read_line(&mut val)
    .expect("Erro ao tentar ler o primeiro numero");
  println!("Computando...");
  sleep(Duration::from_millis(1200));
  println!("Valor computado!");
  return val.trim()
      .parse::<i64>()
      .expect("Erro ao tentar converter string para i64");
}

fn show_option(val1: i64, val2: i64) {
  println!("-----REFERÊNCIA-----");
  println!("1. SOMA");
  println!("2. SUBTRAÇÃO");
  println!("3. DIVISÃO");
  println!("4. MULTIPLICAÇÃO");
  println!("--------------------");

  let type_calc = get_line_cmd();
  let type_calc = type_calc.trim()
    .parse::<i32>()
    .expect("Erro ao converter o tipo para numeros"); 
  
  if type_calc == 1 {
    println!("Soma entre {} e {} é {}", val1,val2, val1+val2);
  } else if type_calc == 2 {
    println!("Subtração entre {} e {} é {}", val1,val2, val1-val2);
  } else if type_calc == 3 {
    println!("Divisão entre {} e {} é {}", val1,val2, val1/val2);
  } else if type_calc == 4 {
    println!("Multiplicação entre {} e {} é {}", val1,val2, val1*val2);
  } else {
    println!("Desculpa mas não encontramos essa referencia");
    sleep(Duration::from_secs(1));
    show_option(val1, val2);
  }
}

fn main() {
  loop {
    let val1 = computed("Digite o primeiro numero");
    let val2 = computed("Agora é o segundo");
    show_option(val1,val2);
    println!("Deseja sair? (s/N)");
    let exit = get_line_cmd();
    match &exit.trim() as &str {
      "s" => break,
      "n" => continue,
      _ => continue,
    }
  }
}