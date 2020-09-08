TARGETS = frag.spv vert.spv

all: $(TARGETS)

$(TARGETS):
	glslangValidator -V shader.frag
	glslangValidator -V shader.vert

clean:
	rm $(TARGETS)

