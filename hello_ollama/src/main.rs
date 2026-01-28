
use serde::Deserialize;
use std::time::Duration;
//This is a nested object inside thev Model info struc
#[derive(Deserialize, Debug)]
struct TagsResponse {
     models: Vec<ModelInfo>,
}

#[derive(Deserialize, Debug)]
struct ModelInfo{
    name: String,
}

fn list_models_blocking() -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let url = "http://localhost:11434/api/tags";
    // 1) create blocking client
    let client = reqwest::blocking::Client::new();
     // 2) send GET request
    let resp = client.get(url).send()?;
    // 3) Desrilaize  parse JSON into TagsResponse structre
    let data: TagsResponse = resp.json()?;
    let mut model_names = Vec::new();

    for m in data.models{
        model_names.push(m.name);
    }
    return Ok(model_names)


}

/*We’ll do 1A: timeout with Client builder in blocking style, 
and I’ll also show the “no chaining” expanded version so you don’t lose track. */

fn list_models_blocking_builder_with_parameters() -> Result<Vec<String>, Box<dyn std::error::Error>>{
    /*
        Create a ClientBuilder
        Set timeout in builder
        Build a final Client (can fail) → so it returns Result
     */

    let url = "http://localhost:11434/api/tags";
    // normal client, no global timeout
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).timeout(Duration::from_secs(2)).send()?;
    let data:TagsResponse=resp.json()?;
    let mut model_names = Vec::new();
    for model in data.models{
        model_names.push(model.name);
    }
    return Ok(model_names);

}

fn main() {
    println!("Hello, world!");
    //calling function of the model get code
    let result = list_models_blocking();
    match result{
        Ok(m) =>{
            for i in m{
                println!("{}", i);
            }
        },
        Err(e) => println!("Error: {}", e),
    }

    //calling the vbuilder based reqwest client
    let result = list_models_blocking_builder_with_parameters();
    match result{
        Ok(models) =>{
            for model in models{
                println!("Builder with parameters Model: {}", model);
            }
        }
        Err(err)=>{
            println!("Error in builder with parameters: {}", err);
        }
    }
}
