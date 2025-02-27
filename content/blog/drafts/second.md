+++
title = "My second post"
date = 2019-11-28
+++

This is my second blog post.

first, given a list of names of artists, their lyrics are scraped from genius.com. 

```rust
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = match std::env::consts::OS {
        "android" => env::var("ANDROID_DATABASE_URL").expect("ANDROID_DATABASE_URL must be set"),
        _ => env::var("LINUX_DATABASE_URL").expect("LINUX_DATABASE_URL must be set"),
    };
    info!("Connecting to {}", database_url);
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```

then a text corpus is created from the scraped lyrics, and the corpus is cleaned of any bad characters. 

```tsx
<ThemeProvider theme={theme}>
    <CssBaseline />
    <Container maxWidth="md" sx={{ userSelect: "none", cursor: "default" }}>
        <RouterProvider router={router} />
        <Toaster position="bottom-center" reverseOrder={false} />
    </Container>
</ThemeProvider>
```
