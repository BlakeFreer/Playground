module Counter(input clk, rst, output reg [bits-1:0] count);
	parameter bits = 4;
	
	always @(posedge clk or posedge rst) begin
		if(rst) begin
			count <= 0;
		end else begin
			count <= count + 1;
		end;
	end

endmodule