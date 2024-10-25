use warp::fs::dir;
use warp::Filter;

fn generate_svg(
    text: &str,
    start_color: &str,
    end_color: &str,
    foreground_color: &str,
    icon_color: &str,
    border_color: &str,
) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"300\" height=\"60\">
            <defs>
                <linearGradient id=\"grad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">
                    <stop offset=\"0%\" style=\"stop-color:{start_color};stop-opacity:1\" />
                    <stop offset=\"100%\" style=\"stop-color:{end_color};stop-opacity:1\" />
                </linearGradient>
            </defs>
            <rect width=\"300\" height=\"60\" fill=\"url(#grad)\" rx=\"15\" ry=\"15\" stroke=\"{border_color}\" stroke-width=\"3\" />
            <circle cx=\"30\" cy=\"30\" r=\"20\" fill=\"{icon_color}\" />
            <text x=\"70\" y=\"38\" fill=\"{foreground_color}\" font-size=\"20\" font-family=\"Arial, Helvetica, sans-serif\" font-weight=\"bold\">{text}</text>
         </svg>",
        start_color = start_color,
        end_color = end_color,
        foreground_color = foreground_color,
        icon_color = icon_color,
        border_color = border_color,
        text = text
    )
}

#[derive(serde::Deserialize)]
struct BadgeParams {
    text: String,
    start_color: String,
    end_color: String,
    foreground_color: String,
    icon_color: String,
    border_color: String,
}

#[tokio::main]
async fn main() {
    // Define a route that captures query parameters for the SVG generation
    let svg = warp::path("badge")
        .and(warp::query::<BadgeParams>())
        .map(|params: BadgeParams| {
            let svg_content = generate_svg(
                &params.text,
                &params.start_color,
                &params.end_color,
                &params.foreground_color,
                &params.icon_color,
                &params.border_color,
            );
            warp::http::Response::builder()
                .header("content-type", "image/svg+xml")
                .body(svg_content)
        });

    // Define a route to serve static files from the `static` folder
    let static_files = warp::path::end().and(dir("static"));

    // Combine routes to serve both the SVG generation and the HTML file
    let routes = static_files.or(svg);

    // Start the warp server on port 3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// After building and `cargo run`, go to:  http://localhost:3030
