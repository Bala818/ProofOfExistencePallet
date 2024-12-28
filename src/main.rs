mod runtime;
mod proof;

use runtime::Runtime;
use proof::Call;

fn main() {
    let mut runtime = Runtime::<String, String>::new();

    // Create a claim
    runtime
        .execute(Call::CreateClaim {
            caller: "Alice".to_string(),
            content: "Document1".to_string(),
        })
        .unwrap();

    // Revoke a claim
    runtime
        .execute(Call::RevokeClaim {
            caller: "Alice".to_string(),
            content: "Document1".to_string(),
        })
        .unwrap();

    println!("Runtime State: {:?}", runtime);
}
