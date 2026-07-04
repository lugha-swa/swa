// Count AST nodes needed for each msingi .swa file
use kande_lib::lexer::Lexer;
use kande_lib::parser;

fn count_nodes_for_file(path: &str) {
    let src = std::fs::read_to_string(path)
        .expect(&format!("failed to read {}", path));
    let lexer = Lexer::new(&src);
    let tokens = lexer.tokenize();
    let result = parser::parse_full(&tokens);
    match result {
        Ok((_, _, _, _, _, _, _, _, count)) => {
            println!("{:35} {:>6} lines  {:>5} AST nodes  ({:.1} nodes/line)",
                path, src.lines().count(), count,
                if src.lines().count() > 0 { count as f64 / src.lines().count() as f64 } else { 0.0 });
            if count > 4096 {
                println!("  *** EXCEEDS 4096 NODE LIMIT! ***");
            }
        }
        Err(e) => {
            println!("{:35} ERROR: {}", path, e);
        }
    }
}

fn main() {
    println!("AST Node Usage Analysis");
    println!("{}", "-".repeat(80));
    count_nodes_for_file("msingi/kumbukumbu.swa");
    count_nodes_for_file("msingi/mfuatano.swa");
    count_nodes_for_file("msingi/msomaji.swa");
    count_nodes_for_file("msingi/msambazaji.swa");
    count_nodes_for_file("msingi/mteremko.swa");
    count_nodes_for_file("msingi/mkaguzi.swa");
    println!("{}", "-".repeat(80));
    // Combined total (what stage1 actually parses)
    let kumbukumbu = std::fs::read_to_string("msingi/kumbukumbu.swa").unwrap();
    let mfuatano = std::fs::read_to_string("msingi/mfuatano.swa").unwrap();
    let msomaji = std::fs::read_to_string("msingi/msomaji.swa").unwrap();
    let msambazaji = std::fs::read_to_string("msingi/msambazaji.swa").unwrap();
    let mteremko = std::fs::read_to_string("msingi/mteremko.swa").unwrap();
    let mkaguzi = std::fs::read_to_string("msingi/mkaguzi.swa").unwrap();

    let all_source = format!("{}{}{}{}{}{}",
        kumbukumbu, mfuatano, msomaji, msambazaji, mteremko, mkaguzi);
    let lexer = Lexer::new(&all_source);
    let tokens = lexer.tokenize();
    let result = parser::parse_full(&tokens);
    match result {
        Ok((_, _, _, _, _, _, _, _, count)) => {
            let lines = all_source.lines().count();
            println!("{:35} {:>6} lines  {:>5} AST nodes  ({:.1} nodes/line)",
                "ALL COMBINED (full source set)", lines, count,
                count as f64 / lines as f64);
            if count > 4096 {
                println!("  *** EXCEEDS 4096 NODE LIMIT! ***");
            }
        }
        Err(e) => {
            println!("ALL COMBINED ERROR: {}", e);
        }
    }
    println!("{}", "-".repeat(80));
}
