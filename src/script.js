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

    if (data?.descendants?.length > 0) {
        data.descendants.forEach(comment => {
            const commentDiv = document.createElement("div");
            commentDiv.className = "flex items-start space-x-4 p-4 border-full border-gray-800";

            const avatar = document.createElement("img");
            avatar.src = comment.account.avatar;
            avatar.alt = `${comment.account.display_name} avatar`;
            avatar.className = "w-10 h-10 rounded-full";
            commentDiv.appendChild(avatar);

            const contentDiv = document.createElement("div");
            contentDiv.className = "flex-1";

            const authorName = document.createElement("a");
            authorName.className = "font-semibold";
            authorName.textContent = comment.account.display_name;
            authorName.href = comment.account.url;

            const commentText = document.createElement("a");
            commentText.className = "mt-1";
            commentText.innerHTML = comment.content;
            commentText.href = comment.url;

            contentDiv.appendChild(authorName);
            contentDiv.appendChild(commentText);
            commentDiv.appendChild(contentDiv);

            commentsView.appendChild(commentDiv);
        });
    } else {
        const noComments = document.createElement("p");
        noComments.className = "p-4 text-center";
        noComments.textContent = "No comments yet.";
        commentsView.appendChild(noComments);
    }
}