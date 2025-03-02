use yew::prelude::*;  

#[function_component(App)] 
fn app() -> Html {  
    let expr = use_state(|| " ".to_owned());

    let draw = Callback::from({
        let expr = expr.clone();
        move |val: &str|{
            let temp = (*expr).to_owned();
            if temp.contains("ERROR") {
                expr.set(val.to_owned());
            } else {
                let result = (*expr).to_owned() + val;
                expr.set(result);
            }
        }
    });

    let clear = Callback::from({
        let expr = expr.clone();
        move |_| {
            expr.set("".to_owned());
        }
    });

    let calculate = Callback::from({
        let expr = expr.clone();
        move |_: &str| {
            let eval = meval::eval_str((*expr).to_owned());
            match eval {
                Ok(res) => {
                    if res > 1e10 {
                        expr.set(format!("{:+.6e}", res));
                    } else {
                        expr.set(format!("{:.12}", res.to_string()));
                    }
                },
                Err(_) => {expr.set("ERROR".to_owned());}
            }
            
        }
    });

    let log = Callback::from({
        let expr = expr.clone();
        move |_: &str| {
            let eval = meval::eval_str("ln(".to_owned() + (*expr).as_str() + ")");
            match eval {
                Ok(res) => {
                    if res > 1e10 {
                        expr.set(format!("{:+.6e}", res));
                    } else {
                        expr.set(format!("{:.12}", res.to_string()));
                    }
                },
                Err(_) => {expr.set("ERROR".to_owned());}
            }
        }
    });

    let onkeydown = Callback::from({
        let expr = expr.clone();
        move |event: KeyboardEvent| {
            let temp = (*expr).to_owned();
            let val = event.key();
            if temp.contains("ERROR") {
                expr.set(val);
            } else {
                let mut result = (*expr).to_owned();
                match val.as_str() {
                    "Backspace" => {result.pop();},
                    "Enter" => {
                        let eval = meval::eval_str((*expr).to_owned());
                        match eval {
                            Ok(res) => {
                                if res > 1e10 {
                                    result = format!("{:+.6e}", res);
                                } else {
                                    result = format!("{:.12}", res.to_string());
                                };
                            },
                            Err(_) => {result = "ERROR".to_owned();}
                        }
                    },
                    "Escape" => {result = "".to_owned();}

                    "1" => {result.push_str("1");},
                    "2" => {result.push_str("2");},
                    "3" => {result.push_str("3");},
                    "4" => {result.push_str("4");},
                    "5" => {result.push_str("5");},
                    "6" => {result.push_str("6");},
                    "7" => {result.push_str("7");},
                    "8" => {result.push_str("8");},
                    "9" => {result.push_str("9");},
                    "0" => {result.push_str("0");},
                    "." => {result.push_str(".");},

                    "+" => {result.push_str("+");},
                    "-" => {result.push_str("-");},
                    "/" => {result.push_str("/");},
                    "*" => {result.push_str("*");},
                    "^" => {result.push_str("^");},
                    "%" => {result.push_str("%");},
                    "l" => {
                        let eval = meval::eval_str("ln(".to_owned() + (*expr).as_str() + ")");
                        match eval {
                            Ok(res) => {result = format!("{:.12}", res.to_string());},
                            Err(_) => {result = "ERROR".to_owned();}
                        }
                    },
                    _ => {},
                }
                expr.set(result);
            }
        }
    });

    html! {
        <div tabindex="0" onkeydown={onkeydown.clone()}>
            <img class="banner" src="img/calc_banner.png"/>
            <table id="calcu" >      
                <tr>      
                    <td colspan="3">
                        <div class="display_wrapper">
                            <div class="display" >{(*expr).clone()}</div>
                        </div>
                    </td>
                    <td><input class="button" type="button" value="c" onclick={move |_|clear.emit("")} /> </td>      
                </tr>      
                
                <tr>          
                    <td><input class="button" type="button" value="1" onclick={let onclick = draw.clone(); move |_|onclick.emit("1")} /> </td>
                    <td><input class="button" type="button" value="2" onclick={let onclick = draw.clone(); move |_|onclick.emit("2")} /> </td>
                    <td><input class="button" type="button" value="3" onclick={let onclick = draw.clone(); move |_|onclick.emit("3")} /> </td>
                    <td><input class="button" type="button" value="/" onclick={let onclick = draw.clone(); move |_|onclick.emit("/")} /> </td>     
                </tr>

                <tr>          
                    <td><input class="button" type="button" value="4" onclick={let onclick = draw.clone(); move |_|onclick.emit("4")} /> </td>
                    <td><input class="button" type="button" value="5" onclick={let onclick = draw.clone(); move |_|onclick.emit("5")} /> </td>
                    <td><input class="button" type="button" value="6" onclick={let onclick = draw.clone(); move |_|onclick.emit("6")} /> </td>
                    <td><input class="button" type="button" value="*" onclick={let onclick = draw.clone(); move |_|onclick.emit("*")} /> </td>  
                    <td><input class="button" type="button" value="L" onclick={let onclick = log.clone(); move |_|onclick.emit("")} /> </td>
                </tr>

                <tr>          
                    <td><input class="button" type="button" value="7" onclick={let onclick = draw.clone(); move |_|onclick.emit("7")} /> </td>
                    <td><input class="button" type="button" value="8" onclick={let onclick = draw.clone(); move |_|onclick.emit("8")} /> </td>
                    <td><input class="button" type="button" value="9" onclick={let onclick = draw.clone(); move |_|onclick.emit("9")} /> </td>
                    <td><input class="button" type="button" value="-" onclick={let onclick = draw.clone(); move |_|onclick.emit("-")} /> </td>  
                    <td><input class="button" type="button" value="%" onclick={let onclick = draw.clone(); move |_|onclick.emit("%")} /> </td>
                </tr>

                <tr>          
                    <td><input class="button" type="button" value="0" onclick={let onclick = draw.clone(); move |_|onclick.emit("0")} /> </td>
                    <td><input class="button" type="button" value="." onclick={let onclick = draw.clone(); move |_|onclick.emit(".")} /> </td>
                    <td><input class="button" type="button" value="=" onclick={let onclick = calculate.clone(); move |_|onclick.emit("")} /> </td>
                    <td><input class="button" type="button" value="+" onclick={let onclick = draw.clone(); move |_|onclick.emit("+")} /> </td>  
                    <td><input class="button" type="button" value="^" onclick={let onclick = draw.clone(); move |_|onclick.emit("^")} /> </td>
                </tr>

            </table>     
        </div>
        
    } 
}  

fn main() {
    yew::Renderer::<App>::new().render(); 
}