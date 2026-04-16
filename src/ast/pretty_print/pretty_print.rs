use crate::ast::ast::*;
use std::fmt::Write;

pub struct PrettyPrinter<W: Write> {
    out: W,
}

impl<W: Write> PrettyPrinter<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }

    pub fn finish(self) -> W {
        self.out
    }

    fn indent(&mut self, n: usize) {
        for _ in 0..n {
            write!(self.out, "    ").unwrap();
        }
    }

    fn print_body(&mut self, cmds: &[AndOr], indent: usize) {
        for cmd in cmds {
            self.print_and_or(cmd, indent + 1);
        }
    }

    pub fn print_complete(&mut self, c: &CompleteCommand) {
        self.print_list(&c.list, 0);
    }
    fn print_list(&mut self, l: &List, indent: usize) {
        self.print_and_or(&l.first, indent);
        for op in &l.rest {
            self.print_and_or(&op.node, indent);
        }
    }

    fn print_pipeline(&mut self, p: &Pipeline, indent: usize) {
        if p.commands.is_empty() {
            return;
        }

        self.indent(indent);

        if p.negated {
            write!(self.out, "! ").unwrap();
        }

        for (i, cmd) in p.commands.iter().enumerate() {
            if i != 0 {
                write!(self.out, " | ").unwrap();
            }
            self.print_command(cmd, indent);
        }
    }

    fn print_simple(&mut self, s: &SimpleCommand, _indent: usize) {
        let mut parts: Vec<String> = vec![];

        for a in &s.assignments {
            match &a.value {
                Some(v) => parts.push(format!("{}={}", a.key, v)),
                None    => parts.push(a.key.clone()),
            }
        }
        if let Some(name) = &s.name {
            parts.push(name.clone());
        }
        for w in &s.words {
            parts.push(w.clone());
        }

        write!(self.out, "command: {}", parts.join(" ")).unwrap();
    }

    fn print_and_or(&mut self, a: &AndOr, indent: usize) {
        self.print_pipeline(&a.first, indent);

        for node in &a.rest {
            self.indent(indent);
            match node.op {
                AndOrOp::AndIf => write!(self.out, " &&\n").unwrap(),
                AndOrOp::OrIf  => write!(self.out, " ||\n").unwrap(),
            }
            self.print_pipeline(&node.rhs, indent + 1);
        }

        writeln!(self.out).unwrap();
    }

    fn print_compound(&mut self, c: &CompoundCommand, indent: usize) {
        match c {
            CompoundCommand::If(i)    => self.print_if(i, indent),
            CompoundCommand::While(w) => self.print_while(w, indent),
            CompoundCommand::Until(u) => self.print_until(u, indent),
            CompoundCommand::For(f)   => self.print_for(f, indent),
            CompoundCommand::Case(c)  => self.print_case(c, indent),

            CompoundCommand::Subshell(cmds) => {
                write!(self.out, "command substitution: (").unwrap();
                if cmds.is_empty() {
                    writeln!(self.out, ")").unwrap();
                } else {
                    writeln!(self.out).unwrap();
                    for cmd in cmds {
                        self.print_and_or(cmd, indent + 1);
                    }
                    self.indent(indent);
                    writeln!(self.out, ")").unwrap();
                }
            }

            CompoundCommand::BraceGroup(cmds) => {
                writeln!(self.out, "command block: {{").unwrap();
                for cmd in cmds {
                    self.print_and_or(cmd, indent + 1);
                }
                self.indent(indent);
                writeln!(self.out, "}}").unwrap();
            }
        }
    }

    fn print_if(&mut self, i: &IfClause, indent: usize) {
        writeln!(self.out, "if").unwrap();
        for cond in &i.condition {
            self.print_and_or(cond, indent + 1);
        }

        self.indent(indent);
        writeln!(self.out, "then").unwrap();
        for then in &i.then_branch {
            self.print_and_or(then, indent + 1);
        }

        for elif in &i.elif_parts {
            self.indent(indent);
            writeln!(self.out, "elif").unwrap();
            for cond in &elif.condition {
                self.print_and_or(cond, indent + 1);
            }
            self.indent(indent);
            writeln!(self.out, "then").unwrap();
            for then in &elif.then_branch {
                self.print_and_or(then, indent + 1);
            }
        }

        if let Some(els) = &i.else_branch {
            self.indent(indent);
            writeln!(self.out, "else").unwrap();
            for e in els {
                self.print_and_or(e, indent + 1);
            }
        }

        self.indent(indent);
        writeln!(self.out, "fi").unwrap();
    }

    fn print_while(&mut self, w: &WhileClause, indent: usize) {
        writeln!(self.out, "while").unwrap();
        for cond in &w.condition {
            self.print_and_or(cond, indent + 1);
        }
        self.indent(indent);
        writeln!(self.out, "do").unwrap();
        for b in &w.body {
            self.print_and_or(b, indent + 1);
        }
        self.indent(indent);
        writeln!(self.out, "done").unwrap();
    }

    fn print_until(&mut self, u: &UntilClause, indent: usize) {
        writeln!(self.out, "until").unwrap();
        for cond in &u.condition {
            self.print_and_or(cond, indent + 1);
        }
        self.indent(indent);
        writeln!(self.out, "do").unwrap();
        for b in &u.body {
            self.print_and_or(b, indent + 1);
        }
        self.indent(indent);
        writeln!(self.out, "done").unwrap();
    }

    fn print_command(&mut self, c: &Command, indent: usize) {
        match c {
            Command::Simple(s) => self.print_simple(s, indent),
            Command::Compound { command, redirects } => {
                self.print_compound(command, indent);
                self.print_redirects(redirects);
            }
            Command::Function(f) => self.print_function(f, indent),
        }
    }

    fn print_redirects(&mut self, r: &[Redirect]) {
        for redir in r {
            let op = match redir.kind {
                RedirectKind::Input      => "<",
                RedirectKind::Output     => ">",
                RedirectKind::Append     => ">>",
                RedirectKind::HereDoc    => "<<",
                RedirectKind::HereDocDash => "<<-",
                RedirectKind::DupInput   => "<&",
                RedirectKind::DupOutput  => ">&",
                RedirectKind::Clobber    => ">|",
                RedirectKind::Open       => "<>",
            };

            let target = redir.target.as_deref().unwrap_or("???");

            match redir.fd {
                Some(fd) => {
                    write!(self.out, " {}{}{}", fd, op, target).unwrap();
                }
                None => {
                    write!(self.out, " {}{}", op, target).unwrap();
                }
            }

            if let (RedirectKind::HereDoc | RedirectKind::HereDocDash, Some(body)) =
                (redir.kind, &redir.body)
            {
                write!(self.out, " [heredoc body: {} lines]", body.lines().count()).unwrap();
                writeln!(self.out, "\n{}", body).unwrap();
            }
        }
    }

    fn print_for(&mut self, f: &ForClause, indent: usize) {
        writeln!(self.out, "for").unwrap();

        self.indent(indent + 1);
        writeln!(self.out, "{}", f.var).unwrap();

        self.indent(indent);
        writeln!(self.out, "in").unwrap();

        if let Some(words) = &f.words {
            self.indent(indent + 1);
            for w in words {
                write!(self.out, "{} ", w).unwrap();
            }
            writeln!(self.out).unwrap();
        }

        self.indent(indent);
        writeln!(self.out, "do").unwrap();

        for b in &f.body {
            self.print_and_or(b, indent + 1);
        }
    }

    fn print_case(&mut self, c: &CaseClause, indent: usize) {
        self.indent(indent);
        writeln!(self.out, "case {} in", c.word).unwrap();

        for item in &c.items {
            self.indent(indent + 1);

            let patterns = item.patterns.join(" | ");
            writeln!(self.out, "{})", patterns).unwrap();

            for cmd in &item.body {
                self.print_and_or(cmd, indent + 2);
            }

            self.indent(indent + 1);
            writeln!(self.out, ";;").unwrap();
        }

        self.indent(indent);
        writeln!(self.out, "esac").unwrap();
    }

    fn print_function(&mut self, f: &FunctionDef, indent: usize) {
        self.indent(indent);
        writeln!(self.out, "{}()", f.name).unwrap();
        self.print_compound(&f.body, indent);
        self.print_redirects(&f.redirects);
    }
}