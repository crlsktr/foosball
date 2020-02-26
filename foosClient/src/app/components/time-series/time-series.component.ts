import { Component, OnInit, Input } from '@angular/core';
import { HistoryPoint } from './history-point';
import * as d3 from 'd3';

@Component({
	selector: 'app-time-series',
	templateUrl: './time-series.component.html',
	styleUrls: ['./time-series.component.scss']
})
export class TimeSeriesComponent implements OnInit {
  @Input('data')
  public stats : HistoryPoint[];

	private margin: { left: number, right: number, top: number, bottom: number }
		= { left: 20, right: 20, top: 20, bottom: 20 };
	private width: number = 1600; //need to get from the graph container
	private height: number = 550; //need to get from the graph container
	private axisWidth = { x: 20, y: 30 };

	constructor() { }

	ngOnInit() {

	}

	drawHistory(hist: HistoryPoint[]) {
		//get the boundaries for the graph
		let boundary = document.getElementById('graph').getBoundingClientRect();
		this.width = boundary.width;
		this.height = boundary.height;

		//Determine scaling functions for dimensions on screen
		let xScale = d3
			.scaleTime()
			.domain(d3.extent(hist, x => x.played))
			.range([(this.margin.left + this.axisWidth.y), this.width - this.margin.right]);
		let yScale = d3
			.scaleLinear()
			.domain([d3.min(hist, x => x.ranking), d3.max(hist, x => x.ranking)])
			.nice()
			.range([this.height - (this.margin.bottom + this.axisWidth.x), this.margin.top]);

		//Determine axis tracing functions
		let xAxis = (g: d3.Selection<d3.BaseType, {}, HTMLElement, any>) =>
			g
				.attr('transform', `translate(0, ${this.height - (this.margin.bottom + this.axisWidth.x)})`)
				.call(
					d3
						.axisBottom(xScale)
						.ticks(this.width / 80)
						.tickFormat((value: Date) => value.getMonth() + "/" + value.getDate())
						.tickSizeOuter(0)
				)
				.selectAll('text')
				.attr('transform', 'rotate(-90)') //ticks for dates are rotated -90 due to cramming
				.attr('dx', '-.8em')
				.attr('dy', '-0.15em')
				.style('text-anchor', 'end');

		let yAxis = (g: d3.Selection<d3.BaseType, {}, HTMLElement, any>) =>
			g
				.attr('transform', `translate(${this.margin.left + this.axisWidth.y},0)`)
				.call(
					d3
						.axisLeft(yScale)
						.ticks(8)
				);

		let svg = d3.select("#graph");
		svg.selectAll("*").remove();

		//Trace axis
		svg.append('g').call(xAxis);
		svg.append('g').call(yAxis);

		//Determine line generator function (could replace with another type of graph)
		let lineGenerator = d3.line<HistoryPoint>()
			.x(d => xScale(d.played))
			.y(d => yScale(d.ranking));

		//Trace lines between dots
		svg
			.append('path')
			.datum(hist)
			.attr('stroke', '#f2a003')
			.attr('fill', 'none')
			.attr('stroke-width', '3px')
			.attr('d', lineGenerator(hist));

		//Trace data points (circles)
		svg.selectAll()
			.data(hist)
			.enter()
			.append('circle')
			.attr('r', '5px')
			.attr('cx', (d: HistoryPoint) => xScale(d.played))
			.attr('cy', (d: HistoryPoint) => yScale(d.ranking))
			.attr('fill', '#f2a003')
			.on('mouseover', d => {
				this.highlightGame(d);
			})
			.on('mouseout', d => {
				this.hideGame(d);
			});

		// let yScale
	}


	hideGame(d: HistoryPoint) {
		console.log("xkcd, game to find", d.played.toISOString());
		//This horrible comparison hack is necessary due to the fact that js sucks at setting a consistent standard for Time precision
		//Also TS can't enforce type safety at runtime thus, the serialization from the server's response doesn't account for the type
		//definition of the field as `played_on: Date`

		// let game = this.stats.find(x => new Date(x.played_on).toLocaleString() === d.played.toLocaleString());
		// game.highlight = true;
		// setTimeout(()=>game.highlight = false, 2000)
	}

	highlightGame(d: HistoryPoint) {
		console.log("xkcd, game to find", d.played.toISOString());
		// let game = this.stats.find(x => new Date(x.played_on).toLocaleString() === d.played.toLocaleString());
		// game.highlight = false;
	}
}
