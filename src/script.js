async function load_comments(postid) {
    console.log("LOAD COMMENTS, ID: " + postid)
    const url = `https://mastodon.social/api/v1/statuses/${postid}/context`;
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }
        const json = await response.json();
        populate_comments(json);

    } catch (error) {
        alert(`Error loading comments: ${error.message}`);
        console.error(error.message);
    }
}

function populate_comments(data) {
    const commentsView = document.getElementById("comments-view");
    if (!commentsView) {
        console.error("comments-view element not found");
        return;
    }
    commentsView.innerHTML = "";
    if (data && data.descendants && data.descendants.length > 0) {
        data.descendants.forEach(comment => {
            const commentDiv = document.createElement("div");
            commentDiv.classList.add("comment");
            const avatar = document.createElement("img");
            avatar.src = comment.account.avatar;
            avatar.alt = comment.account.display_name + " avatar";
            avatar.classList.add("comment-avatar");
            commentDiv.appendChild(avatar);
            const contentDiv = document.createElement("div");
            contentDiv.classList.add("comment-content");
            contentDiv.innerHTML = comment.content;
            commentDiv.appendChild(contentDiv);
            const authorName = document.createElement("div");
            authorName.classList.add("comment-author");
            authorName.textContent = comment.account.display_name;
            contentDiv.prepend(authorName);
            commentsView.appendChild(commentDiv);
        });
    } else {
        const noComments = document.createElement("p");
        noComments.textContent = "No comments yet.";
        commentsView.appendChild(noComments);
    }
}