use std::env;
use std::process;
use std::process::Command;
use std::time::Instant;
use std::io::{self, Write};  // Usata da legginum()
/// Pulisco schermo terminale
fn clsterm() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}
/// Uso il Crivello di Eratostene.
fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let limit = (n as f64).sqrt() as usize;
    for p in 2..=limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
    }
    (2..=n).filter(|&i| is_prime[i]).collect()
}
/// Restituisce l'n-esimo numero primo.
/// La posizione parte da 1: nsieve(1) restituisce 2.
fn nsieve(n: usize) -> Option<usize> {
    if n == 0 {
        return None;
    }
    let mut limit = 16;
    loop {
        let primes = sieve(limit);
        if primes.len() >= n {
            return Some(primes[n - 1]);
        }
        limit = limit.checked_mul(2)?;
    }
}
/// Restituisce il numero primo in posizione `npos` (posizione da 1).
/// Usa un crivello segmentato e memorizza soltanto i primi di base.
fn bigprime(npos: usize) -> Option<usize> {
    if npos == 0 {
        return None;
    }
    // Gestisce direttamente i primi cinque numeri primi.
    const SMALL: [usize; 5] = [2, 3, 5, 7, 11];
    if npos <= SMALL.len() {
        return Some(SMALL[npos - 1]);
    }
    // Stima superiore: per n >= 6,
    // p_n < n * (ln(n) + ln(ln(n))).
    let nf = npos as f64;
    let estimate = nf * (nf.ln() + nf.ln().ln());
    if !estimate.is_finite() || estimate >= usize::MAX as f64 {
        return None;
    }
    let limit = estimate.ceil() as usize;
    let root = (limit as f64).sqrt() as usize;
    // Primi necessari per eliminare i composti nei segmenti.
    let base = sieve(root);
    // Abbiamo già contato il numero primo 2.
    let mut count = 1usize;
    let segment_odds: usize = 1 << 20;
    let mut low = 3usize;
    while low <= limit {
        // Ogni segmento contiene soltanto numeri dispari.
        let span = 2 * (segment_odds - 1);
        let mut high = low.saturating_add(span).min(limit);
        if high % 2 == 0 {
            high -= 1;
        }
        let len = (high - low) / 2 + 1;
        let mut is_prime = vec![true; len];
        for &p in &base {
            if p == 2 {
                continue;
            }
            if p > high / p {
                break;
            }
            // Primo multiplo di p >= low, ma non inferiore a p².
            let rem = low % p;
            let delta = if rem == 0 { 0 } else { p - rem };
            let Some(first) = low.checked_add(delta) else {
                continue;
            };
            let Some(square) = p.checked_mul(p) else {
                continue;
            };
            let mut multiple = first.max(square);
            // Si marcano soltanto i multipli dispari.
            if multiple % 2 == 0 {
                let Some(next) = multiple.checked_add(p) else {
                    continue;
                };
                multiple = next;
            }
            let step = 2 * p;
            while multiple <= high {
                is_prime[(multiple - low) / 2] = false;
                let Some(next) = multiple.checked_add(step) else {
                    break;
                };
                multiple = next;
            }
        }
        // Conta i primi del segmento e restituisce quello cercato.
        for (i, &prime) in is_prime.iter().enumerate() {
            if prime {
                count += 1;
                if count == npos {
                    return Some(low + 2 * i);
                }
            }
        }
        if high >= limit.saturating_sub(1) {
            break;
        }
        low = high + 2;
    }
    None
}
/// Legge un numero intero positivo, altrimenti restituisce 0.
fn legginum(messaggio: &str) -> usize {
    print!("{}", messaggio);
    io::stdout().flush().expect("Errore di output");
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return 0;
    }
    match input.trim().parse::<usize>() {
        Ok(n) if n >= 1 => n,
        _ => 0,
    }
}
/// Segnala come usare il programma in caso di errori nei parametri
fn print_usage_and_exit(program: &str) -> ! {
    eprintln!("Uso: {} <limit>", program);
    eprintln!("Stampa tutti i Numeri Primi <= <limit>.");
    process::exit(1);
}
/// ------------------ MAIN PROGRAM ------------------ 
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
	clsterm();
    let n: usize = match args.len() {
        1 => 100, // Nessun argomento: valore predefinito
        2 => match args[1].parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Errore: <limit> deve essere un intero positivo.");
                print_usage_and_exit(program);
            }
        },
        _ => print_usage_and_exit(program),
    };
    let start = Instant::now();
    let primes = sieve(n);
    let elapsed = start.elapsed();
	let count = primes.len();
    // Print primes one per line
	println!();
    for p in primes {
        print!("{} ", p);
    }
    println!("\n\nDa {} numeri ho estratto {} Primi in {:.3?}", n, count, elapsed);
// TEST funzione: fn nsieve(n: usize) -> Option<usize>
	println!("\nIl numero primo in posizione {} è {:?}.", count, nsieve(count));
// TEST funzione: fn bigprime(npos: usize) -> Option<usize>	
	loop {
		let numeroin = legginum("\nInserisci la posizione del numero primo [0 ESCI]: ");
	
		if numeroin == 0 {
			eprintln!("\nIl programma è terminato.\n");
			return;
		}
		match bigprime(numeroin) {
			Some(prime) => {
				println!("\nIl numero primo in posizione {} è {}.", numeroin, prime);
			}
			None => {
				eprintln!("Impossibile calcolare il numero primo richiesto.");
			}
		}
	}
}
