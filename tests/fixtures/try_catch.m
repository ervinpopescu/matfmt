function safe_divide(a, b)
try
result=a/b;
disp(result);
catch e
fprintf('Error: %s\n',e.message);
end
end
