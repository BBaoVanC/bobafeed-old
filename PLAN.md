[modeline]: # vim: textwidth=100

# The Plan

## Main Idea

Take a bunch of posts stored in some user-defined data store and display it using
user-defined templates.

Additionally, allow interop with other site generators/renderers/CMS by having a JSON format that can be
read by bobafeed. Then bobafeed keeps track of the state of it so we can tell when CRUD happens, and
submit the right AP events. This option requires the user to manually configure the site generator
to make sure that author URIs line up, and that all the correct info is shown.

And with that second option: bobafeed has to have some system to provide comments like a
conventional comment system does, so it can be embedded in the static site.

- should accept any base URL
- make path for actor URIs, tags, etc configurable and manually defined

---

Assuming we go with the second option, there are two ways the website owner might want to submit
content. One is via their CMS, good for blog posts and main microblog posts (or maybe even replies).
The other is smaller replies that don't need to be highlighted as much, such as replies to comments
on a blog post. The latter would have to be stored in bobafeed itself. But where do we store those?
Maybe under the author URI with a sequential ID kind of like Twitter/Mastodon.

We have to figure out how to determine whether to put the comment in the CMS or in bobafeed. It's
probably best to leave it up to the user but I need to think of how I want to do it on bbaovanc.com.

For now, this feels like the only things that should be in bobafeed instead of in Hugo:

- replies to comments on blog posts
- retweeting
- likes/favorites
- bookmarks

Replies to posts on the wider fediverse probably make sense to put in Hugo.

## Questions

- can we support multiple data store backends? I could imagine using SQL for microblogging, but git
  repo for main blog posts on bbaovanc.com. if I used bobafeed to write the entire website. maybe
  instead just do interop with other platforms, see below
- how can we do interop between other platforms? so you aren't forced to use bobafeed to render your
  website just to get it published on the fediverse.
  - example could be with Hugo, and maybe just leave it up to the user to hardcode the correct URLs
    (such as the author URI on my website for example)
    - and then have some sort of JSON format to communicate what the content of the page is to be
      parsed by bobafeed (I do not want to implement microformats scraping), make it automatically
      rendered by Hugo as a different output format
    - bobafeed will have to be a bit more stateful to remember what posts are and figure out
      when/whether they're CRUD-ed
