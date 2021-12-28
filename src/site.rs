// async fn get_site() -> Result<(), Box<dyn std::error::Error>> {
//     let base_url = "https://coomer.party";
//     let url = "https://coomer.party/artists";

//     let site_page_len = get_page_num(&url).await?;
//     let site_bar = get_pbar(
//         site_page_len as u64,
//         "Site coom.party [{elapsed_precise}] {bar:40.cyan/blue} {pos:1}/{len:5}",
//     )?;

//     for i in (0..site_page_len).progress_with(site_bar) {
//         let site_pages = gen_links(&url, "user-card__name", i, base_url).await?;
//         let artists_bar = get_pbar(
//             site_pages.len() as u64,
//             "Artists [{elapsed_precise}] {bar:40.cyan/blue} {pos:1}/{len:5}",
//         )?;

//         for site_page in site_pages.iter().progress_with(artists_bar) {
//             let artist_name = site_page.split("/").last().unwrap();

//             get_pages(site_page, base_url, artist_name).await?;
//         }
//     }
//     Ok(())
// }