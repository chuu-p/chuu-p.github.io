#!/usr/bin/env fish

# New post script (wizard)
# - title of new post? 
# - description of new post? 
# - you want a miku quote? Y/n
# 	- which miku Img? Select from N
# 	- which miku quote? Type in
# - you want Mastodon comments? Y/n
# 	- Create a Mastodon toot with toot cli with (TODO) chuu.dev/link to the new post, title and description of the post 
# 	- remember the ID and put that in the MDX blog post template 

function read_confirm -a prompt
    while true
        read -l -P $prompt confirm
        switch $confirm
            case Y y
                return 0
            case '' N n
                return 1
        end
    end
end

read -l -P "enter slug of new post: " post_slug
read -l -P "enter title of new post: " post_title
read -l -P "enter description of new post: " post_description

set header ""

if read_confirm "miku quote? [y/N] "
    read -l -P "enter miku img: " post_miku_img
    read -l -P "enter miku quote: " post_miku_quote

    set header "
miku: true
mikuImg: \"$post_miku_img\"
mikuQuote: \"$post_miku_quote\"
"
end

if read_confirm "mastodon comments? [y/N] "
    set toot_content "New Blog-Post: $post_title!"\n\n"$post_description"\n\n"https://chuu.dev/blog/$post_title"

    set toot_resp (toot post $toot_content)
    # set toot_resp "Toot posted: https://mastodon.social/@chuu_p/114548027542551647"

    set toot_split (string split / $toot_resp)
    set toot_id $toot_split[-1]

    set header "$header
comments: true
commentsTootId: \"$toot_id\""
end

set header "$header

tags: ['tag1', 'tag2']
---
"

set header_sep "tags: ['tag1', 'tag2']
---"

# echo $header

npm run new $post_slug

set content (cat ./src/content/blog/$post_slug.md | string collect)
set new_content (string replace -- $header_sep $header $content | string collect)

# echo old content $content
# echo new content $new_content
# echo $new_content >./src/content/blog/$post_slug.md
mv ./src/content/blog/$post_slug.md ./src/content/blog/$post_slug.mdx
echo created ./src/content/blog/$post_slug.mdx successfully!
