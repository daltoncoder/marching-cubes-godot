extends Node3D

func _ready():
	# Wait a frame to ensure the extension is loaded
	await get_tree().process_frame
	demo_sphere()
	demo_terrain()
	demo_custom_data()
	demo_animated_terrain()

func demo_sphere():
	# Create a sphere using marching cubes
	var generator = MarchingCubesGenerator.new()
	var mesh = generator.generate_sphere(32, 10.0, 0.1)
	
	# Create a MeshInstance3D to display it
	var mesh_instance = MeshInstance3D.new()
	mesh_instance.mesh = mesh
	mesh_instance.position = Vector3(0, 0, 0)
	
	# Add material
	var material = StandardMaterial3D.new()
	material.albedo_color = Color.BLUE
	mesh_instance.material_override = material
	
	add_child(mesh_instance)

func demo_terrain():
	# Generate terrain
	var generator = MarchingCubesGenerator.new()
	var mesh = generator.generate_terrain(64, 32, 64, 0.5, 0.1, 5.0)
	
	var mesh_instance = MeshInstance3D.new()
	mesh_instance.mesh = mesh
	mesh_instance.position = Vector3(25, -5, -10)
	
	var material = StandardMaterial3D.new()
	material.albedo_color = Color.GREEN
	mesh_instance.material_override = material
	
	add_child(mesh_instance)

func demo_custom_data():
	# Create custom volumetric data
	var width = 32
	var height = 32
	var depth = 32
	var data = PackedFloat32Array()
	data.resize(width * height * depth)
	
	# Fill with custom function (torus shape)
	var center = Vector3(width/2, height/2, depth/2)
	var major_radius = 8.0
	var minor_radius = 3.0
	
	for z in range(depth):
		for y in range(height):
			for x in range(width):
				var pos = Vector3(x, y, z) - center
				
				# Torus distance function
				var xz_dist = Vector2(pos.x, pos.z).length()
				var torus_dist = Vector2(xz_dist - major_radius, pos.y).length()
				var value = torus_dist - minor_radius
				
				var index = z * height * width + y * width + x
				data[index] = value
	
	# Generate mesh
	var generator = MarchingCubesGenerator.new()
	var mesh = generator.generate_mesh(data, width, height, depth, 0.0, 0.2,true)
	
	if mesh:
		var mesh_instance = MeshInstance3D.new()
		mesh_instance.mesh = mesh
		mesh_instance.position = Vector3(-5, -2, 0)
		
		var material = StandardMaterial3D.new()
		material.albedo_color = Color.RED
		mesh_instance.material_override = material
		
		add_child(mesh_instance)

# Real-time example with noise
func demo_animated_terrain():
	var generator = MarchingCubesGenerator.new()
	var mesh_instance = MeshInstance3D.new()
	mesh_instance.name = "TerrainMesh"
	mesh_instance.position = Vector3(-10, -15, -10)
	add_child(mesh_instance)
	
	# Update mesh every frame with time-based noise
	var tween = create_tween()
	tween.set_loops()
	tween.tween_method(update_terrain, 0.0, 10.0, 5.0)

func update_terrain(time: float):
	var width = 32
	var height = 16
	var depth = 32
	var data = PackedFloat32Array()
	data.resize(width * height * depth)
	
	# Time-animated noise terrain
	for z in range(depth):
		for y in range(height):
			for x in range(width):
				var noise_val = sin(x * 0.3 + time) * cos(z * 0.3 + time) * 3.0
				var terrain_height = noise_val + height * 0.3  # Terrain height from bottom
				
				# Invert Y coordinate: Y=0 becomes top, Y=height becomes bottom
				var world_y = height - 1 - y
				
				# Density: positive below terrain (inside), negative above terrain (outside)
				var density = terrain_height - world_y
				
				var index = z * height * width + y * width + x
				data[index] = density
	
	var generator = MarchingCubesGenerator.new()
	var mesh = generator.generate_mesh(data, width, height, depth, 0.0, 1.0, true)
	
	if mesh and has_node("TerrainMesh"):
		get_node("TerrainMesh").mesh = mesh
