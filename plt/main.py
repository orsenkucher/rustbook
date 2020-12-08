from flask import Flask, Response, send_file, request
import io
import random
from matplotlib.figure import Figure
from matplotlib.backends.backend_agg import FigureCanvasAgg as FigureCanvas
from flask_cors import CORS, cross_origin

app = Flask(__name__)
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'

@app.route('/plot', methods=['GET', 'POST'])
@cross_origin()
def plot():
    config = request.json
    fig = empty_figure()
    if config:
      fig = create_figure(config)
    output = io.BytesIO()
    FigureCanvas(fig).print_png(output)
    return Response(output.getvalue(), mimetype='image/png')

def create_figure(config):
    fig = Figure(figsize=(6,6), dpi=240)
    axis = fig.add_subplot(1, 1, 1)
    xs = range(100)
    ys = [random.randint(1, 50) for x in xs]
    axis.plot(xs, ys)
    return fig

def empty_figure():
   return Figure(figsize=(1,1), dpi=40)
