module test(input [3:0] KEY, output [3:0] LEDR);

reg z;

reg [2:0] count;
assign LEDR[3:1] = count;

assign LEDR[0] = z;
always @(negedge KEY[0]) begin
	count = count + 1;
	z <= KEY[0] ^ KEY[1];
end

endmodule