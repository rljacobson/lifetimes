fn longest(x: &str, y: &str) -> &str { // ━━━━━━┱───┐ -x and y borrows here.
    if x.len() > y.len() { //   x's borrow    S4┃ S5│ 
        x     //                overlaps with   ┃   │ -loan_if = x, x drops.
    } else {  //                y's borrow      ┃   │
        y     // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╋───┼ -loan_if = y, y drops.
    } // either x or y is still borrowing here  ┃   │
} // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┹───┘ -result borrows here, while
                                                   //  x and y go out of scope.
fn main() {
    let string1 = String::from(  // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ -string1 born
        "long string is long");  //   string1's scope              S1┃
    let result;                  // ┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉┉╂┉┉┐
    { //    result's scope, but not assigned to, so no lifetime yet  ┃S4┋
        let string2 = String::from("xyz"); //─────────────────────┐  ┃  ┋ -string2 born.
        result = longest(string1.as_str(), // string2's scope   S2│  ┃  ┋
                        string2.as_str());       // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┼┄┄╂┄┄┼┄┄┄┐ -string1 or string2 borrowed here.
    }                                            // ──────────────┘  ┃  ┋ S3┆ -string2 dies.
    println!("The longest string is {}", result);// - S5             ┃  ┋   ┆ -result drops.
                                                 // ━━━━━━━━━━━━━━━━━┹┉┉┘┄┄┄┘ 
}                                                // - S6, global scope.