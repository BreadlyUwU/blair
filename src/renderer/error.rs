pub fn render_error(err: askama::Error) -> String {
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
        <h2>Something went all shitty poop uhh~!!! :c</h2>
        <hr/>
        <p>{:?}</p>
        <hr/>
        <p>Please contact the shitty dev (that's me, I'm the shitty dev) at <a href=\"https://mstdn.breadcat.run\">@Alexa@breadcat.run</a> (Fediverse) or <a href=\"https://bsky.app/profile/breadcat.run\">@breadcat.run</a> (Atmosphere)</p>
    </body>
</html>
", err);
}