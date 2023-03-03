// TODO: should article id be separate
pub struct PostId(pub u32);
/// A blog article
pub struct Article;
/// A short micro-blog post, TODO: subset of article?
pub struct Post {
    /// the post, if any, that is being replied to
    reply_to: Option<PostId>,
    content: String,
}


pub struct CommentId(pub u32);
pub struct Comment {
    parent: Option<CommentId>,
    content: String,

}
