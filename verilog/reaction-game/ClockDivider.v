module ClockDivider(input clk_in, output clk_out);
	parameter prescaler = 3; // f_out = f_in / (prescaler + 1)
	
	reg [$clog2(prescaler+1):0] counter; // N+1 since counter will reach N
	
	assign clk_out = counter == 0;
		
	always @(posedge clk_in) begin				
		counter = counter + 1;
		if(counter >= prescaler) begin
			counter = 0;
		end
	end
endmodule