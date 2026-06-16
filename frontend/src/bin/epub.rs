use epub_builder::{EpubBuilder, EpubContent, MetadataOpf, ReferenceType, ZipLibrary};
use frontend::digte;
use std::fs::File;
use uuid::Uuid;

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

const MY_NAMESPACE: Uuid = uuid::uuid!("a1b2c3d4-e5f6-7890-abcd-ef1234567890");

const YEARS: [(&str, &str, &str); 31] = [
    ("Brønden", "1965", "9788785590008"),
    ("Natur", "1968–1970", "9788785590015"),
    ("Halvfjerdserbrun", "1972", "9788785590022"),
    ("Stengrunds tørst", "1980–1983", "9788785590039"),
    ("Lys i mørket", "1986", "9788785590046"),
    ("Hjertets rytme", "2001", "9788785590053"),
    ("Klodeskyer", "2002", "9788785590060"),
    ("Stilhedens styrke", "2002", "9788785590077"),
    ("Yrk", "2002–2003", "9788785590084"),
    ("Konserves", "2003", "9788785590091"),
    ("Vækst", "2003", "9788785590107"),
    ("Firs tekster", "2003–2004", "9788785590114"),
    ("Kun et øjeblik", "2004", "9788785590121"),
    ("Dagen", "2004", "9788785590138"),
    ("Det ender med et smil", "2004", "9788785590145"),
    ("Vejen", "2004", "9788785590152"),
    ("Til Dig", "2004", "9788785590169"),
    ("Undervejs", "2004", "9788785590176"),
    ("Filibuster", "2004–2005", "9788785590183"),
    ("Perler på snor", "2004–2010", "9788785590190"),
    ("Billedbogen", "2005", "9788785590206"),
    ("Hjertets slag", "2005", "9788785590213"),
    ("Grib", "2005", "9788785590220"),
    ("Overvejelser", "2005", "9788785590237"),
    ("Skygger", "2005–2006", "9788785590244"),
    ("Tegn", "2005–2006", "9788785590251"),
    ("Jordens liv", "2006", "9788785590268"),
    ("Opdagelsesrejse", "2006", "9788785590275"),
    ("Månelys", "2006–2009", "9788785590282"),
    ("Aftryk", "2010–2011", "9788785590299"),
    ("Landskab", "2004–2023", "9788785590305"),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (samling_name, year, isbn) in YEARS.iter() {
        let filename = format!("books/{}.epub", samling_name.replace(" ", "_"));
        let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;

//        let cover_page = format!(
//            r#"<html xmlns="http://www.w3.org/1999/xhtml">
//    <head><title>Forside</title><link rel="stylesheet" type="text/css" href="../stylesheet.css"/></head>
//    <body style="margin:0; padding:0; text-align:center;">
//        <img src="cover.png" alt="{samling_name}"
//             style="max-width:100%; max-height:100vh; display:block; margin:0 auto;"/>
//    </body>
//    </html>"#
//        );
        let cover_page = format!(
            r#"<html xmlns="http://www.w3.org/1999/xhtml">
            <head>
                <title>Forside</title>
                <link rel="stylesheet" type="text/css" href="stylesheet.css"/>
            </head>
            <body>
                <div style="text-align:center;">
                    <img src="cover.png" alt="{samling_name}"
                         style="max-width:100%; max-height:100vh; display:block; margin:0 auto;"/>
                </div>
            </body>
            </html>"#
            );

        builder.add_content(
            EpubContent::new("cover.xhtml", cover_page.as_bytes()).reftype(ReferenceType::Cover),
        )?;

        let author = "Frede Østergaard";
        builder.metadata("toc_name", "Indholdsfortegnelse")?;

        // UUID v5: deterministically derived from a namespace + name
        // Same input always produces the same UUID
        //builder.metadata("identifier", isbn)?
        let uuid = Uuid::new_v5(&MY_NAMESPACE, samling_name.as_bytes());
        builder.set_uuid(uuid);
        builder.add_metadata_opf(Box::new(MetadataOpf {
            name: "dc:identifier".to_string(),
            content: isbn.to_string(),
        }));

        let cover_file = format!("covers/cover_{}.png", samling_name.replace(" ", "_"));
        let image_data =
            std::fs::read(&cover_file).map_err(|_| format!("Not found: {cover_file}"))?;
        builder.add_cover_image("cover.png", image_data.as_slice(), "image/png")?;

        // Metadata
        builder.metadata("author", author)?;
        builder.metadata("title", *samling_name)?;
        builder.add_language("da");

        let css = "
            /* Force the reader to use a single column layout */
            html {
                -webkit-column-count: 1 !important;
                column-count: 1 !important;
            }

            body {
                margin: 5%;
                font-family: serif;
                line-height: 1.6;
            }

            .poem {
                /* Force a page break before every poem */
                page-break-before: always;
                break-before: page;
                
                /* Center the poem vertically for a 'premium' feel */
                display: flex;
                flex-direction: column;
                justify-content: center;
                min-height: 80vh; 
            }

            .theme-list { 
                font-size: 0.85em; 
                color: #666; 
                font-style: italic; 
                margin-top: 3em; 
                border-top: 1px solid #eee;
                padding-top: 1em;
            }

            h1 { text-align: center; margin-top: 20%; }

            .author-photo img { max-width: 60%; display: block; margin: 1em auto; }

            .copyright {
                margin-top: 40%;
                font-size: 0.85em;
                color: #555;
            }

            .copyright p {
                margin: 0.3em 0;
            }

        ";

        builder.stylesheet(css.as_bytes())?;
        //builder.add_resource("stylesheet.css", css.as_bytes(), "text/css")?;

        let title_page = format!(
            r#"<html xmlns="http://www.w3.org/1999/xhtml">
    <head><title>{samling_name}</title><link rel="stylesheet" type="text/css" href="stylesheet.css"/></head>
    <body>
        <h1>{}</h1>
        <h2>Frede Østergaard</h2>
    </body>
    </html>"#,
            samling_name
        );

        builder.add_content(
            EpubContent::new("title.xhtml", title_page.as_bytes())
                .title(*samling_name)
                .reftype(ReferenceType::TitlePage),
        )?;

        let copyright_page = format!(
r#"<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Kolofon</title>
    <link rel="stylesheet" type="text/css" href="stylesheet.css"/>
</head>
<body>
    <div class="copyright">
        <p>{samling_name}</p>

        <p>© Frede Østergaard {year}</p>
        <p>Digtene er skrevet: {year}</p>

        <p>EPUB-udgave udgivet i 2026</p>
        <p>Forlaget Den Gamle Poet</p>

        <p>ISBN: {isbn}</p>

        <p>Kildekode: github.com/jesper-olsen/poeten</p>

        <p>Alle rettigheder forbeholdes</p>
    </div>
</body>
</html>"#
);

        builder.add_content(
            EpubContent::new("copyright.xhtml", copyright_page.as_bytes())
                .reftype(ReferenceType::Copyright),
        )?;

        builder.inline_toc();

        // 1. Add Foreword (Forord)
        //let forord_content = format!(
        //    r#"<html xmlns="http://www.w3.org/1999/xhtml">
        //    <head><title>Forord</title></head>
        //    <body>
        //        <h1>Forord</h1>
        //        <p>Her kan du skrive din indledning til samlingen {}.</p>
        //    </body>
        //    </html>"#,
        //    samling_name
        //);
        //builder.add_content(
        //    EpubContent::new("forord.xhtml", forord_content.as_bytes())
        //        .title("Forord")
        //        .reftype(ReferenceType::Preface),
        //)?;

        // 2. Filter and add poems for this collection
        let samling_poems = digte::DIGTE
            .iter()
            .enumerate()
            .filter(|(_, (name, _, _))| *name == *samling_name);

        for (i, (_, _temaer_mask, tekst)) in samling_poems {
            let first_line = tekst.lines().next().unwrap_or("Uden titel");

            // Format Themes
            //let mut theme_labels = Vec::new();
            //for (idx, label) in digte::TEMAER.iter().enumerate() {
            //    if (1u64 << idx) & temaer_mask != 0 && *label != "Oplæsning" {
            //        theme_labels.push(*label);
            //    }
            //}
            //let themes_html = if !theme_labels.is_empty() {
            //    format!(
            //        r#"<div class="theme-list"><br/><br/>Temaer: {}</div>"#,
            //        theme_labels.join(", ")
            //    )
            //} else {
            //    String::new()
            //};

            // Format Poem body
            let tekst = xml_escape(&tekst);
            let poem_body = tekst.replace("\n", "<br/>\n");

            let content = format!(
                r#"<html xmlns="http://www.w3.org/1999/xhtml">
                <head>
                    <link rel="stylesheet" type="text/css" href="stylesheet.css"/>
                    <title>{first_line}</title>
                </head>
                <body>
                    <div class="poem">
                        {poem_body}
                    </div>
                </body>
                </html>"#
            );

            builder.add_content(
                EpubContent::new(format!("poem_{}.xhtml", i), content.as_bytes())
                    .title(first_line)
                    .reftype(ReferenceType::Text),
            )?;
        }

        let image_data = std::fs::read("Images/Pasfoto 1964.jpg")?;
        builder.add_resource("author.jpg", image_data.as_slice(), "image/jpeg")?;

        let about = r#"<?xml version="1.0" encoding="utf-8"?>
        <html xmlns="http://www.w3.org/1999/xhtml">
        <head>
            <link rel="stylesheet" type="text/css" href="stylesheet.css"/>
            <title>Om forfatteren</title>
        </head>
        <body>
            <h1>Om forfatteren</h1>
            <div class="author-photo">
                <img src="author.jpg" alt="Frede Østergaard" />
            </div>

            <p>
            Frede Østergaard (f. 1945) er lærer og teologisk uddannet.
            Han begyndte at skrive digte tidligt i livet, og
            forfatterskabet har udviklet sig over mere end seks årtier.
            </p>

            <p>
            De første tekster blev skrevet og delt i mindre,
            uformelle sammenhænge, før de senere blev samlet og
            udgivet gennem <i>Den gamle Poet</i>.
            Siden er der udkommet en lang række digtsamlinger.
            </p>

            <h2>Kontakt</h2>
            <p>
            <a href="mailto:dengamlepoet@gmail.com">dengamlepoet@gmail.com</a>
            </p>
        </body>
        </html>
        "#;

        builder.add_content(
            EpubContent::new("om_forfatteren.xhtml", about.as_bytes())
                .title("Om forfatteren")
                .reftype(ReferenceType::Text),
        )?;

        let back_matter = r#"<html xmlns="http://www.w3.org/1999/xhtml">
    <head>
        <link rel="stylesheet" type="text/css" href="stylesheet.css"/>
        <title>Andre samlinger</title>
    </head>
    <body>
        <h1>Andre samlinger</h1>

        <p>Frede Østergaard har desuden udgivet:</p>

        <ul>
          <li>Brønden (1965)</li>
          <li>Natur (1968–1970)</li>
          <li>Halvfjerdserbrun (1972)</li>
          <li>Stengrunds tørst (1980–1983)</li>
          <li>Lys i mørket (1986)</li>
          <li>Hjertets rytme (2001)</li>
          <li>Klodeskyer (2002)</li>
          <li>Stilhedens styrke (2002)</li>
          <li>Yrk (2002–2003)</li>
          <li>Konserves (2003)</li>
          <li>Vækst (2003)</li>
          <li>Firs tekster (2003–2004)</li>
          <li>Kun et øjeblik (2004)</li>
          <li>Dagen (2004)</li>
          <li>Det ender med et smil (2004)</li>
          <li>Vejen (2004)</li>
          <li>Til Dig (2004)</li>
          <li>Undervejs (2004)</li>
          <li>Filibuster (2004–2005)</li>
          <li>Perler på snor (2004–2010)</li>
          <li>Billedbogen (2005)</li>
          <li>Hjertets slag (2005)</li>
          <li>Grib (2005)</li>
          <li>Overvejelser (2005)</li>
          <li>Skygger (2005–2006)</li>
          <li>Tegn (2005–2006)</li>
          <li>Jordens liv (2006)</li>
          <li>Opdagelsesrejse (2006)</li>
          <li>Månelys (2006–2009)</li>
          <li>Aftryk (2010–2011)</li>
          <li>Landskab (2004–2023)</li>
        </ul>
    </body>
    </html>"#;

        builder.add_content(
            EpubContent::new("andre_samlinger.xhtml", back_matter.as_bytes())
                .title("Andre samlinger")
                .reftype(ReferenceType::Text),
        )?;

        // 3. Generate the file
        let mut file = File::create(&filename)?;
        builder.generate(&mut file)?;
        println!("Generated: {filename}");
    }

    Ok(())
}
