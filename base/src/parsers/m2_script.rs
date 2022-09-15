// Standard USes

// Crate Uses
// use crate::pest::Parser;

// External Uses
// use anyhow::Result;
// use pest::iterators::Pair;


#[derive(Parser)]
#[grammar = "parsers/m2_script.pest"]
pub struct M2ScriptParser;

/*
pub fn parse_file(contents: &str) -> Result<M2Script> {
    let script = M2ScriptParser::parse(Rule::script, contents)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> M2Script {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        match pair.as_rule() {
            Rule::object => M2Script::Group(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::group => todo!(),
            // Rule::list => M2Script::List(pair.into_inner().map(parse_value).collect()),
            Rule::key => M2Script::String(pair.into_inner().next().unwrap().as_str()),
            Rule::string => M2Script::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => M2Script::Number(pair.as_str().parse().unwrap()),

            _ => unreachable!(),
        }
    }

    Ok(parse_value(script))
}

*/

