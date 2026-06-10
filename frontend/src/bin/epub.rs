use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use frontend::digte;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for samling_name in digte::SAMLINGER {
        let filename = format!("books/{}.epub", samling_name.replace(" ", "_"));
        let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;

        let cover_file = format!("covers/cover_{}.png", samling_name.replace(" ", "_"));
        if let Ok(image_data) = std::fs::read(&cover_file) {
            builder.add_cover_image("cover.png", image_data.as_slice(), "image/png")?;
        } else {
            let s = format!("Not found: {cover_file}");
            panic!("{s}")
        }

        // Metadata
        let title = format!("{samling_name}");
        builder.metadata("author", "F. Østergaard")?;
        builder.metadata("title", title)?;
        builder.metadata("lang", "da")?;

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
        ";

        builder.stylesheet(css.as_bytes())?;

        let title_page = format!(
            r#"<html xmlns="http://www.w3.org/1999/xhtml">
    <body>
        <h1>{}</h1>
        <h2>F. Østergaard</h2>
    </body>
    </html>"#,
            samling_name
        );

        builder.add_content(
            EpubContent::new("title.xhtml", title_page.as_bytes())
                .title(samling_name)
                .reftype(ReferenceType::TitlePage),
        )?;

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
            .filter(|(_, (name, _, _))| *name == samling_name);

        for (i, (_, temaer_mask, tekst)) in samling_poems {
            let first_line = tekst.lines().next().unwrap_or("Uden titel");

            // Format Themes
            let mut theme_labels = Vec::new();
            for (idx, label) in digte::TEMAER.iter().enumerate() {
                if (1u64 << idx) & temaer_mask != 0 && *label != "Oplæsning" {
                    theme_labels.push(*label);
                }
            }
            let themes_html = if !theme_labels.is_empty() {
                format!(
                    r#"<div class="theme-list"><br/><br/>Temaer: {}</div>"#,
                    theme_labels.join(", ")
                )
            } else {
                String::new()
            };

            // Format Poem body
            let poem_body = tekst.replace("\n", "<br/>\n");

            let content = format!(
                r#"<html xmlns="http://www.w3.org/1999/xhtml">
                <head><title>{first_line}</title></head>
                <body>
                    <div class="poem">
                        {poem_body}
                    </div>
                    {themes_html}
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

        let about = r#"
        <html xmlns="http://www.w3.org/1999/xhtml">
        <head><title>Om forfatteren</title></head>
        <body>
        <h1>Om forfatteren</h1>
        <div class="author-photo">
        <img src="author.jpg" alt="F. Østergaard" />
        </div>

        <p>
        F. Østergaard debuterede med digtsamlingen
        <i>Brønden</i> i 1965.
        Efter de tidlige samlinger fulgte et længere ophold,
        inden forfatterskabet blev genoptaget i 2002.
        Siden er der udkommet en lang række digtsamlinger.
        </p>

        </body>
        </html>
        "#;

        builder.add_content(
            EpubContent::new("om_forfatteren.xhtml", about.as_bytes())
                .title("Om forfatteren")
                .reftype(ReferenceType::Text),
        )?;

        let back_matter = format!(
            r#"<html xmlns="http://www.w3.org/1999/xhtml">
    <head>
        <title>Andre samlinger</title>
    </head>
    <body>
        <h1>Andre samlinger</h1>

        <p>F. Østergaard har desuden udgivet:</p>

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
    </html>"#
        );

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
