use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    goto: &'a str,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PartCategory {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Rule<'a> {
    category: PartCategory,
    operation: LessOrGreater,
    number: u32,
    goto: &'a str,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum LessOrGreater {
    /// \<
    Less,
    /// \>
    Greater,
}

impl<'a> Rule<'a> {
    fn goto_or_next(&self, part: Part) -> Option<&'a str> {
        let v = match self.category {
            PartCategory::X => part.x,
            PartCategory::M => part.m,
            PartCategory::A => part.a,
            PartCategory::S => part.s,
        };
        let valid = match self.operation {
            LessOrGreater::Less => v < self.number,
            LessOrGreater::Greater => v > self.number,
        };
        valid.then_some(self.goto)
    }
}

impl<'a> Workflow<'a> {
    fn goto(&self, part: Part) -> &'a str {
        for rule in self.rules.iter() {
            if let Some(next) = rule.goto_or_next(part) {
                return next;
            }
        }
        self.goto
    }
}

pub fn part1(input: &str) -> Answer {
    let (_, (workflow, parts)) = parse(input)?;

    let mut result = 0;

    for part in parts {
        let mut current = "in";
        while let Some(wf) = workflow.get(current) {
            current = wf.goto(part);
            if current == "A" || current == "R" {
                break;
            }
        }
        if current == "A" {
            result += part.sum();
        }
    }

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, (workflow, _)) = parse(input)?;

    let parts =
        iproduct!(1..=4000, 1..=4000, 1..=4000, 1..=4000).map(|(x, m, a, s)| Part { x, m, a, s });
    let mut result: u64 = 0;

    for part in parts {
        let mut current = "in";
        while let Some(wf) = workflow.get(current) {
            current = wf.goto(part);
            if current == "A" || current == "R" {
                break;
            }
        }
        if current == "A" {
            result += part.sum() as u64;
        }
    }

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, (BTreeMap<&str, Workflow>, Vec<Part>)> {
    let mut it = iterator(
        input,
        terminated(tuple((alpha1, parse_workflow)), line_ending),
    );
    let workflow = it.collect();
    let (input, _) = it.finish()?;
    let (input, _) = multispace1(input)?;
    let (input, parts) = separated_list1(line_ending, parse_part)(input)?;
    Ok((input, (workflow, parts)))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, (rules, goto)) = delimited(
        tag("{"),
        separated_pair(separated_list1(tag(","), parse_rule), tag(","), alpha1),
        tag("}"),
    )(input)?;
    Ok((input, Workflow { rules, goto }))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, category) = alt((
        tag("x").map(|_| PartCategory::X),
        tag("m").map(|_| PartCategory::M),
        tag("a").map(|_| PartCategory::A),
        tag("s").map(|_| PartCategory::S),
    ))(input)?;
    let (input, operation) = alt((
        tag(">").map(|_| LessOrGreater::Greater),
        tag("<").map(|_| LessOrGreater::Less),
    ))(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, goto) = alpha1(input)?;
    Ok((
        input,
        Rule {
            category,
            operation,
            number,
            goto,
        },
    ))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = complete::u32(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = complete::u32(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = complete::u32(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = complete::u32(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, Part { x, m, a, s }))
}
