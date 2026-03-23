% This is a test file for the matlab-formatter.

function [output] = my_function(input1, input2)
    % This is a comment.
    a = 1+2;
    b = [1,2,3];
    c = { 'a' , 'b' , 'c' };

    if a > 0
        disp('positive');
    else
        disp('non-positive');
    end

    for i = 1:10
        disp(i);
    end
end
