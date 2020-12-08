from flask import Flask, Response, send_file, request
import io
import random
import matplotlib.pyplot as plt
from matplotlib.figure import Figure
from matplotlib.backends.backend_agg import FigureCanvasAgg as FigureCanvas
from flask_cors import CORS, cross_origin

app = Flask(__name__)
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'
plt.style.use('bmh')

@app.route('/plot/<index>', methods=['GET', 'POST'])
@cross_origin()
def plot(index):
    config = request.json
    fig = Figure()
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
    fig.tight_layout()
    return fig
