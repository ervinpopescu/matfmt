% Top-level comment
function foo()
% line comment
a = 1;

%{
  This is a block comment.
  It should not be reformatted.
    Weird    spacing   preserved.
%}

% formatter ignore 2
x=1+  2;
y=3*   4;

z = 5 + 6;
end
