use std::fmt::Write;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Node {
    token: Token,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn coll(&self) -> String {
        let mut s = String::new();

        if let Some(ref lhs) = self.lhs {
            s = format!("({})", lhs.coll()) + &s;
        }

        s += &format!(" {} ", self.token.tos());

        if let Some(ref rhs) = self.rhs {
            s = s + &format!("({})", rhs.coll());
        }

        s
    }

    fn eval(&self, vars: &Vec<(char, bool)>) -> bool {
        match self.token {
            Token::And =>
                self.lhs.as_ref().unwrap().eval(vars) && self.rhs.as_ref().unwrap().eval(vars),
            Token::Or =>
                self.lhs.as_ref().unwrap().eval(vars) || self.rhs.as_ref().unwrap().eval(vars),
            Token::Not =>
                !self.rhs.as_ref().unwrap().eval(vars),
            Token::If => {
                let lhs = self.lhs.as_ref().unwrap().eval(vars);
                let rhs = self.rhs.as_ref().unwrap().eval(vars);

                if !lhs {
                    true
                } else {
                    rhs
                }
            },
            Token::Iff => {
                let lhs = self.lhs.as_ref().unwrap().eval(vars);
                let rhs = self.rhs.as_ref().unwrap().eval(vars);

                lhs == rhs
            },
            Token::ParOpen => unreachable!(),
            Token::ParClose => unreachable!(),
            Token::True => true,
            Token::False => false,
            Token::Var(c) => {
                let res = vars.iter()
                    .find(|(var, _val)| c == *var);

                match res {
                    Some((_var, val)) => *val,
                    None => panic!("Undefined variable: {}", c),
                }
            },
        }
    }

    fn unknowns(&self, vars: &Vec<(char, bool)>, unks: &mut HashSet<char>) {
        if let Token::Var(c) = self.token {
            if !vars.iter().any(|(var, _val)| *var == c) {
                unks.insert(c);
            }
        }

        if let Some(ref lhs) = self.lhs {
            lhs.unknowns(vars, unks);
        }

        if let Some(ref rhs) = self.rhs {
            rhs.unknowns(vars, unks);
        }
    }

    fn unknown_vars(&self, vars: &Vec<(char, bool)>) -> HashSet<char> {
        let mut unks = HashSet::new();
        self.unknowns(vars, &mut unks);
        unks
    }
}

// | 2 4 3 4   1   34  2  34  |
//   !(P v Q) <=> (!P) ^ (!Q)
#[derive(Clone, Debug, PartialEq)]
enum Token {
    And,
    Or,
    Not,
    If,
    Iff,
    ParOpen,
    ParClose,

    True,
    False,
    Var(char),
}

impl Token {
    // To String (char)
    fn tos(&self) -> char {
        match self {
            Token::And => '&',
            Token::Or => '|',
            Token::Not => '!',
            Token::If => '>',
            Token::Iff => '=',
            Token::ParOpen => '(',
            Token::ParClose => ')',

            Token::True => '1',
            Token::False => '0',
            Token::Var(c) => *c,
        }
    }
}

fn is_var(c: char) -> bool {
    c >= 'A' && c <= 'Z' || c >= 'a' && c <= 'z'
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut ret = Vec::new();

    let mut pt = String::new();

    for (i, c) in s.chars().enumerate() {
        if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            continue;
        }

        write!(pt, "{}", c).unwrap();

        ret.push(match pt.as_str() {
            "<=>" => Token::Iff,
            "=>" => Token::If,
            "!" => Token::Not,
            "v" => Token::Or,
            "^" => Token::And,
            "1" => Token::True,
            "0" => Token::False,
            "(" => Token::ParOpen,
            ")" => Token::ParClose,
            "<" | "<=" | "=" => continue,
            _ => {
                if pt.len() == 1 && is_var(c) {
                    Token::Var(c)
                } else {
                    eprintln!("{}", s);

                    let msg = format!("Invalid character");
                    let spaces = " ".repeat(i);
                    eprintln!("{}^ {}", spaces, msg);

                    std::process::exit(1);
                }
            },
        });

        pt.clear();
    }

    // Validation

    let mut par_level = 0;
    let mut var_last = false;
    let mut oper_last = false;
    let mut not_last = false;

    for token in ret.iter() {
        match token {
            Token::Iff | Token::If | Token::Or | Token::And => {
                if oper_last {
                    eprintln!("Expected value after operator");
                    std::process::exit(2);
                }

                if !var_last {
                    eprintln!("Expected value before operator");
                    std::process::exit(2);
                }

                oper_last = true;
                var_last = false;
                not_last = false;
            },
            Token::Not => {
                if var_last {
                    eprintln!("Not operator must come before value, not after");
                    std::process::exit(2);
                }

                oper_last = false;
                var_last = false;
                not_last = true;
            },
            Token::ParOpen => {
                if var_last {
                    eprintln!("Parentheses may not come after a value");
                    std::process::exit(2);
                }

                par_level += 1;

                var_last = false;
                oper_last = false;
                not_last = false;
            },
            Token::ParClose => {
                if oper_last || not_last {
                    eprintln!("Closing parentheses may not come after an operator");
                    std::process::exit(2);
                }

                par_level -= 1;

                if par_level < 0 {
                    eprintln!("No matching parentheses to close");
                    std::process::exit(2);
                }

                var_last = true;
                oper_last = false;
                not_last = false;
            },
            Token::True | Token::False | Token::Var(_) => {
                if var_last {
                    eprintln!("Can't have two values after each other");
                    std::process::exit(2);
                }

                var_last = true;
                oper_last = false;
                not_last = false;
            },
        }
    }

    if oper_last || !var_last {
        eprintln!("Expected value at the end");
        std::process::exit(2);
    }

    if par_level != 0 {
        eprintln!("Unbalanced parentheses");
        std::process::exit(2);
    }

    ret
}

// Evaluate a slice ("array") of tokens
// and create a binary tree of operators, values and variables.
fn eval(s: &[Token]) -> Node {
    // Single token left
    if s.len() == 1 {
        return Node {
            token: s[0].clone(),
            lhs: None,
            rhs: None,
        };
    }

    // Tokens wrapped in parentheses. e.g. (A ^ B) or (P)
    if s.len() >= 3 && s[0] == Token::ParOpen && s[s.len() - 1] == Token::ParClose {
        return eval(&s[1..s.len() - 1]);
    }

    loop {
        // Find the operator which takes least precedence
        let lowest = {
            let mut pos = 0;
            let mut score = 1000;
            let mut parlevel = 0;
            // Iff = 1, If = 2, Or = 3, And = 4, Not = 5

            for i in 0..s.len() {
                let s = match s[i] {
                    Token::ParOpen => { parlevel += 1; continue; },
                    Token::ParClose => { parlevel -= 1; continue; },
                    Token::Iff => parlevel * 10 + 1,
                    Token::If =>  parlevel * 10 + 2,
                    Token::Or =>  parlevel * 10 + 3,
                    Token::And => parlevel * 10 + 4,
                    Token::Not => parlevel * 10 + 5,
                    _ => continue,
                };

                if s < score {
                    score = s;
                    pos = i;
                }
            }

            pos
        };

        let op = s[lowest].clone();

        // Evaluate the 'lowest' operator and (depending on the case)
        // recursively call on the resulting two sub slices.
        match op {
            Token::Not
                => return Node {
                    token: s[lowest].clone(),
                    lhs: None,
                    rhs: Some(Box::new(eval(&s[lowest+1..]))),
            },
            Token::And | Token::Or | Token::If | Token::Iff
                => return Node {
                    token: s[lowest].clone(),
                    lhs: Some(Box::new(eval(&s[..lowest]))),
                    rhs: Some(Box::new(eval(&s[lowest+1..]))),
            },
            Token::True | Token::False | Token::Var(_)
                => return Node {
                    token: s[lowest].clone(),
                    lhs: None,
                    rhs: None,
            },
            _ => unreachable!("Invalid token at this point in execution"),
        }
    }
}

fn main() {
    let lause = std::env::args()
        .nth(1)
        .unwrap_or("1 ^ 1".to_string());

    let tokenized = tokenize(&lause);
    println!("tokenized: {:?}", &tokenized);

    let tree = eval(&tokenized);

    println!("tree: {}", tree.coll());

    // Unused feature, giving variables values.
    let vars = Vec::new();

    let unks = tree.unknown_vars(&vars).iter().map(|c| *c).collect::<Vec<char>>();
    for var in unks.iter() {
        println!("Unknown variable: {}", var);
    }

    if unks.is_empty() {
        println!("eval: {}", tree.eval(&vars));
    } else {
        let mut tautology = true;

        for i in 0..(1 << unks.len()) {
            let mut tvars = Vec::new();
            tvars.extend(vars.iter().map(|(var, val)| (*var, *val)));

            for (j, var) in unks.iter().enumerate() {
                tvars.push((*var, (i & (1 << j)) != 0));
            }

            println!("tvars: {:?}", &tvars);
            if !tree.eval(&tvars) {
                tautology = false;
                break;
            }
        }

        if tautology {
            println!("Is a tautology");
        } else {
            println!("Is not a tautology");
        }
    }
}
