
GPPPARAMS = -m32

%.o: %.cpp
	g++ $(GPPPARAMS) -o $@ -c $<

%.o: %.s
	as $(ASPARAMS) -o $@ $<