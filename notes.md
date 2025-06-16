# Notes, sometimes opinionated.

## Doing a scrolling grid require a complete write of a widget for it.

If you put more boxes than there is space available, the grid will not fold as a scrollable entity.
I haven't digged into it too much, I just think it's better anyway if you want to make something like that to have a very customized widget anyway.
For example an infinite scrolling that generate elements on the fly, usually with a transition animation, oh also with more liberty on the size element.

## Button component doesn't seem to be worthwhile to be used.

I forgetted it and it didn't change a thing. I think it's a bit stuck between a lot of different vision. It's supposed to block something with the picking, but I think that part is bugged or something.
 
## If you want to make custom button geometry.
 
For example, a triangle shaped button.
Don't use Bevy_ui for that, It can only do squares and squares with max border that make it look like a circle.
If you want to do any very gaemy UI with banger graphics, there's two ways:  

- Shaders, a lot of them, more and more shaders. Which you will apply Signed distance functions (SDF) stuff to it. for example https://github.com/viridia/quill/blob/main/crates/bevy_quill_obsidian_graph/src/assets/draw_path.wgsl quill does that

- Use bevy_lunex, It's ui, but not rendered within the UI context, instead it uses the 2D world, has its own camera and stuff. Just check what kind of banger you can do with it https://github.com/IDEDARY/Bevypunk

## The perfect project structure.
 
If really you want the best one

1: Be annoyed by the answer.

2: Actually check that https://github.com/TheBevyFlock/bevy_new_2d and make yourself your own opinion.

3: Just don't bother too much, in rust, project restructuring is a CI/CD pipelining annoyance but that's fine, we're no longer in the age of the C makefile nightmares. 

## Bevy UI is not bad, but it's nowhere nearby something complete.

Expect a lot of moment where two things are similar but distant, to be in fact "merged back" down the line. If you're frowning reading this then it kinda the point I wanted to make.

The ecosystem may appear centralized with a strong maintainer team, centered around cart glorious visions. But is not. Things goes upstream if they are great and accepted by anyone. And things previously integreated gets removed because they got replaced by another system.