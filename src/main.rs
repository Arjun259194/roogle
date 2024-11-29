fn intro() {
    let title = String::from(
        "
 ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░      ░▒▓████████▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░        
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░        
░▒▓█▓▒▒▓███▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒▒▓███▓▒░▒▓█▓▒░      ░▒▓██████▓▒░   
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░        
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░        
 ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓████████▓▒░▒▓████████▓▒░",
    );

    let h1 = String::from("Welcome to WebSearchTUI!");
    let p1 = String::from( "A fast and easy way to search the web directly from your terminal. Forget the hassle of switching between windows or tabs – everything you need is just a few keystrokes away.");
    let h2 = String::from("With WebSearchTUI, you can:");
    let points = vec![
        "Search popular search engines right from your terminal.",
        "Easily switch between search engines using simple commands.",
        "Enjoy a clean and efficient interface that keeps you focused on your search.",
        "Save time and stay productive without leaving your terminal.",
    ];
    let h3 = String::from("Get started:");
    let p2 = String::from( "Just type your query and hit Enter. You’ll instantly see results without the need for a web browser.");

    let h4 = String::from("Features:");
    let points2 = vec![
        "Search with Google, DuckDuckGo, or Bing.",
        "Instant results preview in your terminal.",
        "Navigate through search results using keyboard shortcuts.",
    ];

    println!(
        "
{0}

{1}
{2}

{3}
{4}

{5}
{6}

{7}
{8}

",
        title,
        h1,
        p1,
        h2,
        points
            .iter()
            .map(|p| format!("○ {p}"))
            .collect::<Vec<String>>()
            .join("\n"),
        h3,
        p2,
        h4,
        points2
            .iter()
            .map(|p| format!("• {p}"))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
fn main() {
    intro();
}
