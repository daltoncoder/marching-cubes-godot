# Marching Cubes Godot Extension
https://github.com/user-attachments/assets/b36d7fb0-dec9-46d7-a22d-643154389e67
A simple Godot extension for generating mesh with marching cube algorithm to create anything. Great for procederal or destructable terrains

### To use 
- In your godot project create the following path `res://addons/marching_cubes`  
- Move extensions.gdextension from this repo to that folder  
- run `cargo build --release`  
- move build output from `./target/release` to your marching cubes addon folder you created. Godot should automatically detect plugin but you may need to reload project.   

You can acess the MarchingCubesGenerator Class from your scripts now. Check example folder for an example of what that would look like 


