extends Node3D

@export var height: int = 300
@export var width: int = 300
@export var depth: int = 300
@export var noise_scale: float = 0.1
@export var iso: float = 0.1

var mesh: MeshInstance3D
var noise:  FastNoiseLite
func _ready():	
	var my_mesh = MeshInstance3D.new()
	my_mesh.name = "TerrainMesh"
	add_child(my_mesh)
	mesh = my_mesh
	
	new_seed()
	generate()
	
func new_seed():
	noise = FastNoiseLite.new()
	noise.seed = randi()  # Random seed each time
	#noise.frequency = noise_scale
	noise.noise_type = FastNoiseLite.TYPE_PERLIN
	
func generate():
	var data: PackedFloat32Array
	data.resize(height * width * depth)
	
	for x in range(width):
		for y in range(height):
			for z in range(depth):
				var index = z * height * width + y * width + x 
				var value = noise.get_noise_3d(x, y, z)
				data[index] = value
	
	var generator = MarchingCubesGenerator.new()
	var mesh = generator.generate_mesh(data, width, height, depth, iso, noise_scale, true)
	
	if mesh and has_node("TerrainMesh"):
		get_node("TerrainMesh").mesh = mesh
