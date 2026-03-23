classdef MyClass
properties
x
y
end

methods
function obj = MyClass(a, b)
obj.x = a;
obj.y = b;
end

function r = add(obj)
r=obj.x+obj.y;
end
end
end
