function result = matops(A, B)
% Matrix operations demo
C = [1 2 3;
     4 5 6;
     7 8 9];

D = {A
     B
     C};

result = A * B + C .* D{3};

M = [1 0 0;
0 1 0;
0 0 1];

v = [1, 2, 3];
end
