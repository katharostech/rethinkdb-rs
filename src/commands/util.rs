use crate::{Client, IntoArg};
use protobuf::repeated::RepeatedField;
use ql2::proto::Term;
use ql2::proto::Term_TermType;

pub fn new_client() -> Client {
    Client {
        term: Ok(Term::new()),
        query: String::from("r"),
        write: false,
    }
}

pub fn make_cmd<A: IntoArg>(
    client: &Client,
    name: &'static str,
    cmd_type: Option<Term_TermType>,
    args: Option<A>,
) -> Client {
    let cterm = match client.term {
        Ok(ref term) => term.clone(),
        Err(_) => {
            return client.clone();
        }
    };
    let mut term = Term::new();
    if let Some(cmd_type) = cmd_type {
        term.set_field_type(cmd_type);
    }
    if cterm != Term::new() {
        let prev_cmd = RepeatedField::from_vec(vec![cterm.clone()]);
        term.set_args(prev_cmd);
    }
    let mut cmd = Client::new();
    cmd.term = Ok(term);
    match args {
        Some(args) => {
            let arg = args.into_arg();
            cmd.query = format!("{}.{}({})", client.query, name, arg.string);
            let aterm = match arg.term {
                Ok(term) => term,
                Err(error) => {
                    cmd.term = Err(error);
                    return cmd;
                }
            };
            push_args(&mut cmd, aterm);
        }
        None => {
            cmd.query = format!("{}.{}()", client.query, name);
        }
    }
    log::debug!("{}", cmd.query);
    log::debug!("{:?}", cmd.term);
    cmd
}

fn push_args(cmd: &mut Client, mut tmp_args: Term) {
    if let Ok(ref mut term) = cmd.term {
        if tmp_args.has_field_type() {
            // did not come from the args macro
            term.mut_args().push(tmp_args);
        } else {
            // came from the args macro
            for arg in tmp_args.take_args().into_vec() {
                term.mut_args().push(arg);
            }
            for pair in tmp_args.take_optargs().into_vec() {
                term.mut_optargs().push(pair);
            }
        }
    }
}

pub fn with_args<A: IntoArg>(client: &Client, args: A) -> Client {
    let mut cmd = client.clone();
    if let Err(_) = cmd.term {
        return cmd;
    }
    let args = args.into_arg();
    cmd.query += &format!(".with_args({})", args.string);
    let aterm = match args.term {
        Ok(term) => term,
        Err(error) => {
            cmd.term = Err(error);
            return cmd;
        }
    };
    push_args(&mut cmd, aterm);
    log::debug!("{}", cmd.query);
    log::debug!("{:?}", cmd.term);
    cmd
}
