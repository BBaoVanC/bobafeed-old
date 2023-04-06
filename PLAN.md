[modeline]: # vim: textwidth=100

# The Plan

## Main Idea

Take a bunch of posts stored in some user-defined data store and display it using
user-defined templates.

Additionally, allow interop with other site generators/renderers by having a JSON format that can be
read by bobafeed. Then bobafeed keeps track of the state of it so we can tell when CRUD happens, and
submit the right AP events. This option requires the user to manually configure the site generator
to make sure that author URIs line up, and that all the correct info is shown.

And with that second option: bobafeed has to have some system to provide comments like a
conventional comment system does, so it can be embedded in the static site.

- should accept any base URL
- make path for actor URIs, tags, etc configurable and manually defined

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
