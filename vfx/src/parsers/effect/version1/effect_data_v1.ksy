meta:
  id: effect_data
  title: Effect Data Format
  file-extension: mde
  endian: be
  encoding: utf8

seq:
  - id: magic
    contents: EffectData
  - id: mesh_count
    type: s2
  - id: empty1 # This is here because i don't know how to consume 2 extra bytes
    type: s2
  - id: frame_count
    type: s2
  - id: empty2 # Same as the empty1 field
    type: s2
  - id: meshes
    type: mesh(frame_count)
    repeat: expr
    repeat-expr: mesh_count

types:
  mesh:
    seq:
      - id: object_name
        type: str
        size: 32
      
      - id: diffuse_file_name
        type: str
        size: 128
      
      - id: vertex_count
        type: s2
      - id: empty1
        type: u2
      
      - id: texture_vertex_count
        type: s2
      - id: empty2
        type: u2
      
      - id: index_count
        type: s2
      - id: empty3
        type: u2
      
      - id: frames
        type: frame(vertex_count, index_count, texture_vertex_count)
        repeat: expr
        #repeat-expr: 1
        repeat-expr: frame_count
    params:
      - id: frame_count
        type: u4
  
  frame:
    seq:
      - id: opacity
        type: f4
      
      - id: vertices
        type: v3_axis
        repeat: expr
        #repeat-expr: 1
        repeat-expr: vertex_count
    
      - id: faces
        type: u4
        repeat: expr
        repeat-expr: index_count
    
      - id: texture_vertices
        type: v2_axis
        repeat: expr
        repeat-expr: texture_vertex_count
    
      - id: texture_faces
        type: u4
        repeat: expr
        repeat-expr: index_count
    
    params:
      - id: vertex_count
        type: u4
      - id: index_count
        type: u4
      - id: texture_vertex_count
        type: u4
  
  v2_axis:
    seq:
      - id: x
        type: f4
      - id: y
        type: f4
  
  v3_axis:
    seq:
      - id: x
        type: f4
      - id: y
        type: f4
      - id: z
        type: f4
      
    
  

  