pub fn askama_error(err: askama::Error) -> String { 
    return render_message(
        err.to_string(), 
        Some("Something went wrong during the template's rendering by Askama")
    ); 
}

fn render_message(err: String, opt_message: Option<&str>) -> String {
    return format!("<!doctype html>
<html>
    <head>
        <title>blair::oopsi</title>
        <style>
            :root {{
                color-scheme: light dark;
            }}
        </style>
    </head>
    <body>
        <h1>Oh no!</h1>
        <h2>I'm s… stu… I'm stu… I'm stupiiiid~!!! <span style=\"white-space: nowrap; font-family: monospace;\">(>~<)</span></h2>
        <hr/>
        <p>{:?}<br/>{:?}</p>
        <hr/>
        <p>Please contact the shitty dev (that's me, I'm the shitty dev) at <a href=\"https://breadcat.run/@Alexa\">@Alexa@breadcat.run</a> (Fediverse) or <a href=\"https://bsky.app/profile/breadcat.run\">@breadcat.run</a> (AtProto/Bluesky)</p>
        <p><i>Bweeh</i></p>
    </body>
</html>
", err
 , Some(opt_message).unwrap().ok_or("(No additional information supplied by the error handler)")
);
}