function grade = getGrade(score)
switch score
case 100
grade = 'A+';
case {90, 91, 92, 93, 94, 95, 96, 97, 98, 99}
grade = 'A';
otherwise
grade = 'F';
end
end
