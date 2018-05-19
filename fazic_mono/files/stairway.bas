0 mode 1
10 for i=0 to 99
20 x=rnd(320)
30 y=(rnd(240)+time()*(40+rnd(10)))%240
40 dot x,y
50 next
60 for i=0 to 20
70 for j=0 to 2
80 for k=0 to 0.08 step 0.004
90 a = i/15+k-time()/4
100 y=137-i*8-j
110 line 64+cos(a)*4, 64+cos(a)*40, y+sin(a)*10, 5+j
120 next
130 next
140 next
150 flip
160 goto 10
