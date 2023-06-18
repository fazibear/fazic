0 mode 1
1 color 1 
3 clear 0
4 srand(0)      
60 for i=0 to 25
70 for j=0 to 2
80 for k=0 to 0.60 step 0.004
90 a = i/2 +k-time()
100 y=250-i*12-j
105 color j
106 xx=160+cos(a)*4
107 xxx=160+cos(a)*80
108 yy=y+sin(a)*10
110 line xx, y, xxx, yy
120 next
130 next
140 next
150 flip
160 goto 3 
